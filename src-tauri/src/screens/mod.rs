//! Turns the raw VT100 screen text from a `TuiSession` into a typed
//! `TuiScreen` the frontend can render as native UI instead of a terminal.
//!
//! Every real student connects over SSH as the same shared `estudiante`
//! account (that's transport-layer only -- SSH auth is *not* the real
//! authentication). The demo/no-real-account case drops straight into
//! `MENU PRINCIPAL` and answers account-specific options (like "Seleccion
//! de Secciones") with "Cuenta ESTUDIANTE NO esta disponible por el
//! momento" -- a temporary service-window/maintenance rejection
//! (`TuiScreen::or_err` promotes it to a command error) rather than ever
//! reaching the real per-student flow. With a real account, selecting
//! "Seleccion de Secciones" instead leads to: `Login` (a 4-field
//! ID/PIN/SSN-last-4/birth-date form -- the *actual* authentication,
//! separate from SSH) -> `SelectPeriod` (which semester) -> `Matricula`
//! (the student's course schedule, with an action menu and `Altas`/`Bajas`
//! add/drop-course sub-modes). Grounded in real transcripts captured by
//! the user with a real account: `screens/login.txt` and
//! `screens/matricula/*.txt` at the repo root (that directory can gain new
//! files or be reorganized as the user captures more -- re-check paths
//! rather than assuming). The remote also wraps advisory/event messages in
//! "<< message >>" on a line by itself (distinct from a bracketed key hint
//! embedded mid-instruction, like "Oprima <<Enter>> para Continuar", which
//! is not a notice); `notice::extract_bracketed_notice` catches these as a
//! low-priority fallback for screens not otherwise modeled yet. Anything
//! still not recognized falls back to `Unknown`, which still carries the
//! raw text and any numbered options `scrape::scrape_options` can find, so
//! the app stays usable rather than blocking on it.
//!
//! Each screen type that has dedicated data (`Login`, `SelectPeriod`,
//! `Matricula`) gets its own submodule for its types/scrapers/tests; the
//! `classify` priority chain itself stays in this file rather than being
//! scattered, since the *order* checks run in is a deliberate, tested
//! invariant -- see the module-level comments below for why.

mod login;
mod matricula;
mod notice;
mod scrape;
mod select_period;

use serde::Serialize;

pub use login::LoginField;
pub use matricula::{MatriculaPrompt, ScheduleCourse};

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct MenuOption {
    pub key: String,
    pub label: String,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "kind")]
pub enum TuiScreen {
    /// The `MENU PRINCIPAL:` screen, e.g. "1. Seleccion de Secciones".
    MainMenu { options: Vec<MenuOption> },
    /// The real per-student authentication form (ID number, permanent
    /// access code, last 4 of SSN, birth date) shown after selecting
    /// "Seleccion de Secciones" -- separate from and unrelated to the SSH
    /// login, which every student shares (`estudiante@...`). The remote
    /// warns not to press Enter while filling it in; use `send_text`
    /// (no trailing Enter), not `send_input`.
    Login { fields: Vec<LoginField> },
    /// "Indique Semestre:" -- which term to browse sections for, shown
    /// after a successful `Login`.
    SelectPeriod { options: Vec<MenuOption> },
    /// The student's course schedule (`M A T R I C U L A`) plus whichever
    /// sub-prompt is currently active.
    Matricula {
        courses: Vec<ScheduleCourse>,
        prompt: MatriculaPrompt,
    },
    /// A one-line message shown alongside the current screen, e.g. a
    /// rejection or informational line. Not every `Notice` warrants
    /// treating the action as failed -- see `or_err` for the one pattern
    /// known to mean that today.
    Notice { message: String, raw: String },
    /// The remote's own "<<< P R O C E S O  C O N C L U I D O >>>" banner --
    /// a graceful end of session (seen after logging out, and after the
    /// shared demo account's idle timeout fires). The remote closes the
    /// channel shortly after showing this, so there's nothing left to
    /// interact with; the command layer drops the session when it sees
    /// this rather than leaving the frontend to send input into a dead
    /// connection.
    Disconnected,
    /// Anything not yet recognized. `options` is a best-effort scrape of
    /// numbered lines ("N.  LABEL"), so unmodeled screens are still
    /// navigable from the frontend.
    Unknown { raw: String, options: Vec<MenuOption> },
}

impl TuiScreen {
    /// Turns a rejection meaning "the service you tried to reach is down
    /// right now" into an `Err`, so the command layer doesn't hand the
    /// frontend a dead-end screen to render. Any other screen, including
    /// other `Notice` messages, passes through unchanged.
    pub fn or_err(self) -> Result<TuiScreen, String> {
        if let TuiScreen::Notice { message, .. } = &self {
            let lower = message.to_lowercase();
            if lower.contains("no esta disponible") || lower.contains("no está disponible") {
                return Err(message.clone());
            }
        }
        Ok(self)
    }
}

pub fn classify(raw: &str) -> TuiScreen {
    // Checked before `MENU PRINCIPAL` below: a rejected account-specific
    // option redisplays the main menu *with the notice line still on
    // screen* (grounded in a real transcript), so matching "MENU
    // PRINCIPAL" first would silently swallow the notice.

    // The banner pads every letter with spaces for a marquee effect
    // ("<<<   P R O C E S O    C O N C L U I D O   >>>"), so compare
    // against whitespace-collapsed text rather than matching it literally.
    let collapsed: String = raw.chars().filter(|c| !c.is_whitespace()).collect();
    if collapsed.contains("PROCESOCONCLUIDO") {
        return TuiScreen::Disconnected;
    }

    let notice_line = raw.lines().find(|line| {
        let lower = line.to_lowercase();
        lower.contains("no esta disponible") || lower.contains("no está disponible")
    });
    if let Some(line) = notice_line {
        return TuiScreen::Notice {
            message: line.trim().to_string(),
            raw: raw.to_string(),
        };
    }

    if raw.contains(login::DETECT_HINT) {
        return TuiScreen::Login {
            fields: login::login_fields(),
        };
    }

    if raw.contains("Indique Semestre") {
        return TuiScreen::SelectPeriod {
            options: scrape::scrape_equals_options(raw),
        };
    }

    if raw.contains("M A T R I C U L A") {
        let courses = matricula::scrape_courses(raw);
        let prompt = if raw.contains("[Bajas]") {
            MatriculaPrompt::Bajas
        } else if raw.contains("[Altas]") {
            MatriculaPrompt::Altas
        } else if raw.contains("[Cambio]") {
            MatriculaPrompt::Cambio
        } else {
            MatriculaPrompt::Actions {
                options: scrape::scrape_equals_options(raw),
            }
        };
        return TuiScreen::Matricula { courses, prompt };
    }

    if raw.contains("MENU PRINCIPAL") {
        return TuiScreen::MainMenu {
            options: scrape::scrape_options(raw),
        };
    }

    // Lower priority than the checks above: the remote wraps advisories
    // and event/confirmation messages in "<< ... >>" on screens that
    // aren't specifically modeled yet (e.g. the payment/invoice screen's
    // "Esta factura NO ES OFICIAL..." disclaimer). Checked last, right
    // before the `Unknown` fallback, so it never overrides an
    // already-modeled screen -- unlike the "no esta disponible" notice
    // above, this isn't an action-failure signal, so it shouldn't win over
    // genuinely useful structured content the way that one deliberately
    // does.
    if let Some(message) = notice::extract_bracketed_notice(raw) {
        return TuiScreen::Notice {
            message,
            raw: raw.to_string(),
        };
    }

    TuiScreen::Unknown {
        raw: raw.to_string(),
        options: scrape::scrape_options(raw),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MAIN_MENU: &str = "\
                          UNIVERSIDAD DE PUERTO RICO
                      Recinto Universitario de Mayaguez
18/ago/2026                                                             8:55 am
                         SISTEMA ESTUDIANTIL COLEGIAL


    MENU PRINCIPAL:

       1.  ***>>>  LEE tu Correo Electronico en ->  outlook.com
       2.  Seleccion de Secciones  (Matricula)
       3.  Modificar Codigo de Acceso Permanente
       4.  Informacion Correo Electronico
       5.  Ver otra informacion
       6.  Seleccion de Modalidad P/D/F




       0.  SALIR DEL SISTEMA


Opcion deseada:   ";

    #[test]
    fn classifies_real_main_menu_transcript() {
        let TuiScreen::MainMenu { options } = classify(MAIN_MENU) else {
            panic!("expected MainMenu");
        };
        assert_eq!(options.len(), 7);
        assert_eq!(options[0].key, "1");
        assert_eq!(options[1].label, "Seleccion de Secciones  (Matricula)");
        assert_eq!(options.last().unwrap().key, "0");
        assert_eq!(options.last().unwrap().label, "SALIR DEL SISTEMA");
    }

    #[test]
    fn classifies_account_unavailable_notice() {
        let raw = "              Cuenta ESTUDIANTE NO esta disponible por el momento";
        let screen = classify(raw);
        let TuiScreen::Notice { message, .. } = &screen else {
            panic!("expected Notice");
        };
        assert_eq!(message, "Cuenta ESTUDIANTE NO esta disponible por el momento");
        assert_eq!(
            screen.or_err(),
            Err("Cuenta ESTUDIANTE NO esta disponible por el momento".to_string())
        );
    }

    #[test]
    fn main_menu_passes_through_or_err() {
        assert!(classify(MAIN_MENU).or_err().is_ok());
    }

    #[test]
    fn notice_wins_even_when_menu_principal_is_also_on_screen() {
        // Grounded scenario: selecting an unavailable option redisplays
        // MENU PRINCIPAL with the rejection notice still on screen (its
        // own row, per vt100's one-line-per-terminal-row rendering)
        // rather than replacing it entirely -- the notice must still win.
        let raw =
            format!("              Cuenta ESTUDIANTE NO esta disponible por el momento\n{MAIN_MENU}");
        let screen = classify(&raw);
        assert_eq!(
            screen.or_err(),
            Err("Cuenta ESTUDIANTE NO esta disponible por el momento".to_string())
        );
    }

    #[test]
    fn classifies_proceso_concluido_banner() {
        let raw = "\
UNIVERSIDAD DE PUERTO RICO
                      Recinto Universitario de Mayaguez
18/ago/2026                                                            10:43 am









                <<<   P R O C E S O    C O N C L U I D O   >>>
";
        assert_eq!(classify(raw), TuiScreen::Disconnected);
    }
}
