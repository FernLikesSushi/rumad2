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
//! add/drop-course sub-modes). `MENU PRINCIPAL`'s option 5 ("Ver otra
//! informacion") leads to a separate submenu ("MENU DESPLIEGUE") with its
//! own read-only screens -- `CourseResults` (search results for one
//! course code) and `WeeklySchedule` (the student's own schedule as a
//! day/time grid) -- distinct from `Matricula`'s interactive course list.
//! Grounded in real transcripts captured by the user with a real account:
//! `screens/login.txt`, `screens/matricula/*.txt`, and
//! `screens/menu_despliegue/*.txt` at the repo root (that directory can
//! gain new files or be reorganized as the user captures more -- re-check
//! paths rather than assuming). The remote also wraps advisory/event
//! messages in "<< message >>" on a line by itself (distinct from a
//! bracketed key hint embedded mid-instruction, like "Oprima <<Enter>>
//! para Continuar", which is not a notice); `notice::extract_bracketed_notice`
//! catches these as a low-priority fallback for screens not otherwise
//! modeled yet. Anything still not recognized falls back to `Unknown`,
//! which still carries the raw text and any numbered options
//! `scrape::scrape_options` can find, so the app stays usable rather than
//! blocking on it.
//!
//! Each screen type that has dedicated data (`Login`, `SelectPeriod`,
//! `Matricula`, `CourseResults`, `WeeklySchedule`) gets its own submodule
//! for its types/scrapers/tests; the `classify` priority chain itself
//! stays in this file rather than being scattered, since the *order*
//! checks run in is a deliberate, tested invariant -- see the
//! module-level comments below for why.

mod course_results;
mod interact;
mod login;
mod matricula;
mod notice;
mod scrape;
mod select_period;
mod weekly_schedule;

use serde::Serialize;

pub(crate) use interact::RumadScreen;
pub use course_results::{CourseResultsScreen, CourseSection};
pub use login::{LoginField, LoginScreen};
pub use matricula::{MatriculaPrompt, MatriculaScreen, ScheduleCourse};
pub use select_period::SelectPeriodScreen;
pub use weekly_schedule::{ScheduleRow, WeeklyScheduleScreen};

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct MenuOption {
    pub key: String,
    pub label: String,
}

/// The `MENU PRINCIPAL:` screen, e.g. "1. Seleccion de Secciones".
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct MainMenuScreen {
    pub options: Vec<MenuOption>,
}

/// A one-line message shown alongside the current screen, e.g. a
/// rejection or informational line. Not every `Notice` warrants treating
/// the action as failed -- see `TuiScreen::or_err` for the one pattern
/// known to mean that today.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct NoticeScreen {
    pub message: String,
    pub raw: String,
}

/// Anything not yet recognized. `options` is a best-effort scrape of
/// numbered lines ("N.  LABEL"), so unmodeled screens are still navigable
/// from the frontend.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct UnknownScreen {
    pub raw: String,
    pub options: Vec<MenuOption>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "kind")]
pub enum TuiScreen {
    MainMenu(MainMenuScreen),
    /// The real per-student authentication form (ID number, permanent
    /// access code, last 4 of SSN, birth date) shown after selecting
    /// "Seleccion de Secciones" -- separate from and unrelated to the SSH
    /// login, which every student shares (`estudiante@...`). The remote
    /// warns not to press Enter while filling it in; use `send_text`
    /// (no trailing Enter), not `send_input`.
    Login(LoginScreen),
    /// "Indique Semestre:" -- which term to browse sections for, shown
    /// after a successful `Login`.
    SelectPeriod(SelectPeriodScreen),
    /// The student's course schedule (`M A T R I C U L A`) plus whichever
    /// sub-prompt is currently active.
    Matricula(MatriculaScreen),
    /// Read-only search results for one course code (`MENU DESPLIEGUE` ->
    /// "Horario de cursos disponibles en Matricula") -- every open
    /// section, with room/schedule/professor/capacity. Not the student's
    /// own schedule, just a lookup.
    CourseResults(CourseResultsScreen),
    /// The student's own schedule as a weekly grid (`MENU DESPLIEGUE` ->
    /// "Horario de matricula grafico") -- purely informational, unlike
    /// `Matricula`'s interactive course list.
    WeeklySchedule(WeeklyScheduleScreen),
    Notice(NoticeScreen),
    /// The remote's own "<<< P R O C E S O  C O N C L U I D O >>>" banner --
    /// a graceful end of session (seen after logging out, and after the
    /// shared demo account's idle timeout fires). The remote closes the
    /// channel shortly after showing this, so there's nothing left to
    /// interact with; the command layer drops the session when it sees
    /// this rather than leaving the frontend to send input into a dead
    /// connection.
    Disconnected,
    Unknown(UnknownScreen),
}

impl TuiScreen {
    /// Turns a rejection meaning "the service you tried to reach is down
    /// right now" into an `Err`, so the command layer doesn't hand the
    /// frontend a dead-end screen to render. Any other screen, including
    /// other `Notice` messages, passes through unchanged.
    pub fn or_err(self) -> Result<TuiScreen, String> {
        if let TuiScreen::Notice(NoticeScreen { message, .. }) = &self {
            let lower = message.to_lowercase();
            if lower.contains("no esta disponible") || lower.contains("no está disponible") {
                return Err(message.clone());
            }
        }
        Ok(self)
    }

    /// This screen's `RumadScreen` impl, or `None` for screen kinds that
    /// aren't directly interactive (`Notice` overlays whatever's already
    /// showing rather than being acted on itself; `Disconnected` has no
    /// live session left to send anything to).
    pub(super) fn as_rumad_screen(&self) -> Option<&dyn RumadScreen> {
        match self {
            TuiScreen::MainMenu(s) => Some(s),
            TuiScreen::Login(s) => Some(s),
            TuiScreen::SelectPeriod(s) => Some(s),
            TuiScreen::Matricula(s) => Some(s),
            TuiScreen::CourseResults(s) => Some(s),
            TuiScreen::WeeklySchedule(s) => Some(s),
            TuiScreen::Unknown(s) => Some(s),
            TuiScreen::Notice(_) | TuiScreen::Disconnected => None,
        }
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
        return TuiScreen::Notice(NoticeScreen {
            message: line.trim().to_string(),
            raw: raw.to_string(),
        });
    }

    if raw.contains(login::DETECT_HINT) {
        return TuiScreen::Login(LoginScreen {
            fields: login::login_fields(),
        });
    }

    if raw.contains("Indique Semestre") {
        return TuiScreen::SelectPeriod(SelectPeriodScreen {
            options: scrape::scrape_equals_options(raw),
        });
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
        return TuiScreen::Matricula(MatriculaScreen { courses, prompt });
    }

    if raw.contains("MENU PRINCIPAL") {
        return TuiScreen::MainMenu(MainMenuScreen {
            options: scrape::scrape_options(raw),
        });
    }

    // Checked before `MainMenu`/`Unknown` but after everything above --
    // these two don't share any detection text with another screen, so
    // their exact position among the "specific, grounded" checks doesn't
    // matter, only that they're above the generic bracketed-notice/Unknown
    // fallback below.
    if let Some((course_code, course_title)) = course_results::detect_course(raw) {
        return TuiScreen::CourseResults(CourseResultsScreen {
            course_code,
            course_title,
            sections: course_results::scrape_sections(raw),
        });
    }

    if weekly_schedule::detect(raw) {
        return TuiScreen::WeeklySchedule(weekly_schedule::scrape(raw));
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
        return TuiScreen::Notice(NoticeScreen {
            message,
            raw: raw.to_string(),
        });
    }

    TuiScreen::Unknown(UnknownScreen {
        raw: raw.to_string(),
        options: scrape::scrape_options(raw),
    })
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
        let TuiScreen::MainMenu(MainMenuScreen { options }) = classify(MAIN_MENU) else {
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
        let TuiScreen::Notice(NoticeScreen { message, .. }) = &screen else {
            panic!("expected Notice");
        };
        assert_eq!(
            message,
            "Cuenta ESTUDIANTE NO esta disponible por el momento"
        );
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
        let raw = format!(
            "              Cuenta ESTUDIANTE NO esta disponible por el momento\n{MAIN_MENU}"
        );
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

    // Real (redacted) transcripts captured for a flow that isn't
    // specifically modeled yet -- per this module's own doc comment,
    // `TuiScreen` variants only get written against a real transcript,
    // never guessed ahead of one being captured. These lock in that
    // `classify` still handles them gracefully (no panic, `Unknown` with
    // the raw text preserved) rather than misfiring into an unrelated
    // variant on some incidental substring match. (`horario_estimado.txt`
    // and the `horario_resultados_*.txt` pair *are* modeled now --
    // `weekly_schedule.rs`/`course_results.rs` own their classification
    // tests -- but stay listed below for the "not MainMenu" guard.)
    const HORARIO_CONFIRMADO: &str = include_str!("../../../screens/matricula/horario_confirmado.txt");
    const RESERVA: &str = include_str!("../../../screens/matricula/reserva.txt");
    const MENU_DESPLIEGUE: &str = include_str!("../../../screens/menu_despliegue/menu.txt");
    const HORARIO_SEMESTRE: &str = include_str!("../../../screens/menu_despliegue/horario_semestre.txt");
    const HORARIO_CURSO: &str = include_str!("../../../screens/menu_despliegue/horario_curso.txt");
    const HORARIO_PROCESANDO: &str = include_str!("../../../screens/menu_despliegue/horario_procesando.txt");
    const HORARIO_RESULTADOS_MULTI: &str =
        include_str!("../../../screens/menu_despliegue/horario_resultados_multi.txt");
    const HORARIO_RESULTADOS_SIMPLE: &str =
        include_str!("../../../screens/menu_despliegue/horario_resultados_simple.txt");
    const TURNO_SELECCION: &str = include_str!("../../../screens/menu_despliegue/turno_seleccion.txt");
    const HORARIO_ESTIMADO: &str = include_str!("../../../screens/matricula/horario_estimado.txt");

    #[test]
    fn unmodeled_matricula_screens_fall_back_to_unknown() {
        // Neither shares the `M A T R I C U L A` (spaced) banner
        // `classify` keys off of -- `horario_confirmado.txt`'s own header
        // renders it unspaced ("* MATRICULA ... *"), so it must not
        // accidentally match.
        for raw in [HORARIO_CONFIRMADO, RESERVA] {
            let TuiScreen::Unknown(UnknownScreen { raw: got, .. }) = classify(raw) else {
                panic!("expected Unknown for:\n{raw}");
            };
            assert_eq!(got, raw);
        }
    }

    #[test]
    fn unmodeled_menu_despliegue_screens_fall_back_to_unknown() {
        for raw in [MENU_DESPLIEGUE, HORARIO_SEMESTRE, HORARIO_CURSO, HORARIO_PROCESANDO, TURNO_SELECCION] {
            assert!(
                matches!(classify(raw), TuiScreen::Unknown(_)),
                "expected Unknown for:\n{raw}"
            );
        }
    }

    #[test]
    fn menu_despliegue_screens_dont_get_mistaken_for_main_menu() {
        // "MENU DESPLIEGUE" is option 5's own submenu -- none of its
        // screens (modeled or not) should match the "MENU PRINCIPAL"
        // check and become a `MainMenu`, since its options are a
        // different menu entirely.
        for raw in [
            MENU_DESPLIEGUE,
            HORARIO_SEMESTRE,
            HORARIO_CURSO,
            HORARIO_PROCESANDO,
            HORARIO_RESULTADOS_MULTI,
            HORARIO_RESULTADOS_SIMPLE,
            TURNO_SELECCION,
        ] {
            assert!(
                !matches!(classify(raw), TuiScreen::MainMenu(_)),
                "expected not MainMenu for:\n{raw}"
            );
        }
    }

    #[test]
    fn menu_despliegue_menu_still_scrapes_its_options_via_the_unknown_fallback() {
        // MENU DESPLIEGUE isn't a modeled `MainMenu`, but its numbered
        // options should still come through via `Unknown`'s best-effort
        // scrape, same as `MainMenu`'s -- the whole point of that fallback
        // is that unmodeled screens stay navigable.
        let TuiScreen::Unknown(UnknownScreen { options, .. }) = classify(MENU_DESPLIEGUE) else {
            panic!("expected Unknown");
        };
        assert_eq!(
            options.iter().map(|o| o.key.as_str()).collect::<Vec<_>>(),
            ["1", "2", "3", "4", "5", "6", "7", "8", "9", "0"]
        );
        assert_eq!(options[0].label, "Evaluacion certificacion de Ayuda Economica");
        assert_eq!(options.last().unwrap().label, "Finalizar");
    }

    // `Indique semestre` here is lowercase and shaped differently
    // ("(1=1erVer   2=1erSem ...)" plus a `[PF4=(9)Fin]` hint) from the
    // Matricula flow's own period prompt (`Indique Semestre:`, capital
    // S) that `select_period.rs` models -- a different real screen that
    // happens to share a similar phrase, not the same prompt. Confirms
    // `classify` doesn't conflate the two on a loose substring match.
    #[test]
    fn horario_semestre_is_not_mistaken_for_select_period() {
        assert!(!matches!(classify(HORARIO_SEMESTRE), TuiScreen::SelectPeriod(_)));
    }
}
