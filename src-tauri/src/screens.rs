//! Turns the raw VT100 screen text from a `TuiSession` into a typed
//! `TuiScreen` the frontend can render as native UI instead of a terminal.
//!
//! Only `MainMenu` is grounded in a real transcript (captured by
//! connecting as the shared `estudiante` demo account, which the live
//! system drops straight into `MENU PRINCIPAL`). Account-specific options
//! (class selection, changing the access code) currently answer with
//! "Cuenta ESTUDIANTE NO esta disponible por el momento" -- student-facing
//! services being temporarily unavailable (as of 2026-08-18; a service
//! window/maintenance thing, not a permanent per-account restriction),
//! not because those options require different credentials -- the same
//! options are presumably reachable for administrative account types, and
//! for students once the service is back up. `TuiScreen::as_service_error`
//! flags that specific rejection so the command layer can surface it as an
//! error rather than a dead-end screen; `Notice` itself stays general
//! since other one-line messages may show up under different
//! circumstances and should still render normally. Until someone captures
//! a real transcript of what these options show when the service is up,
//! they aren't modeled as their own `TuiScreen` variants -- anything not
//! yet recognized falls back to `Unknown`, which still carries the raw
//! text and any numbered options `scrape_options` can find, so the app
//! stays usable rather than blocking on it.

use regex::Regex;
use serde::Serialize;
use std::sync::OnceLock;

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
    /// A one-line message shown alongside the current screen, e.g. a
    /// rejection or informational line. Not every `Notice` warrants
    /// treating the action as failed -- see `as_service_error` for the one
    /// pattern known to mean that today.
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

fn option_pattern() -> &'static Regex {
    static PATTERN: OnceLock<Regex> = OnceLock::new();
    // Matches menu lines like "   2.  Seleccion de Secciones  (Matricula)"
    // or "0.  SALIR DEL SISTEMA" -- the numbering style used throughout
    // the real SISTEMA ESTUDIANTIL COLEGIAL screens.
    PATTERN.get_or_init(|| Regex::new(r"(?m)^\s*(\d+)\.\s+(.+?)\s*$").unwrap())
}

fn scrape_options(raw: &str) -> Vec<MenuOption> {
    option_pattern()
        .captures_iter(raw)
        .map(|c| MenuOption {
            key: c[1].to_string(),
            label: c[2].trim().to_string(),
        })
        .collect()
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

    if raw.contains("MENU PRINCIPAL") {
        return TuiScreen::MainMenu {
            options: scrape_options(raw),
        };
    }

    TuiScreen::Unknown {
        raw: raw.to_string(),
        options: scrape_options(raw),
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
