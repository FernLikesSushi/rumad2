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
//! is not a notice); `extract_bracketed_notice` catches these as a
//! low-priority fallback for screens not otherwise modeled yet. Anything
//! still not recognized falls back to `Unknown`, which still carries the
//! raw text and any numbered options `scrape_options` can find, so the
//! app stays usable rather than blocking on it.

use regex::Regex;
use serde::Serialize;
use std::sync::OnceLock;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct MenuOption {
    pub key: String,
    pub label: String,
}

/// One field of the `Login` form, in on-screen order. `label`/`hint` are
/// hardcoded to the correctly-accented Spanish text rather than parsed
/// from the remote
/// screen, because the real transcript's accented characters (í, ó, ú)
/// arrive mangled -- almost certainly a Latin-1/CP437 byte stream getting
/// decoded as UTF-8 somewhere in the vt100/ssh2 pipeline. That's a
/// pre-existing encoding issue worth investigating separately; since this
/// form's labels are static boilerplate (not per-user data), hardcoding
/// clean text sidesteps it rather than trying to fix the pipeline blind.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct LoginField {
    pub key: String,
    pub label: String,
    pub hint: String,
}

/// One row of the student's schedule on the `Matricula` screen, e.g.
/// "1.  INSO 4101      080     3    S". Empty slots (just "N." with
/// nothing after -- the real form has 12) aren't included.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ScheduleCourse {
    pub slot: String,
    pub course: String,
    pub section: String,
    pub credits: String,
    pub status: String,
}

/// Which sub-prompt `Matricula` is currently showing. The header and
/// course list stay identical across these -- only the bottom prompt (and
/// what input it expects) changes.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "kind")]
pub enum MatriculaPrompt {
    /// "Indique: A=Alta B=Baja C=Cambio ..." -- pick a top-level action.
    Actions { options: Vec<MenuOption> },
    /// "Abreviatura y numero de curso  o  FIN" (tagged `[Bajas]`) -- a drop
    /// is in progress; expects a free-text course code, or "FIN" to stop.
    Bajas,
    /// Same prompt shape as `Bajas` (tagged `[Altas]` instead) -- an add is
    /// in progress.
    Altas,
    /// Same prompt shape again (tagged `[Cambio]`) -- a section change is
    /// in progress.
    Cambio,
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

/// Scrapes "KEY=Label" style options, e.g. "1=1er Sem,   2=2do Sem,   3=1er
/// Verano o Verano Extendido" or "A=Alta  B=Baja  ...  O=CodigoReservar
/// S=Salir". Options aren't reliably separated by any fixed run of
/// whitespace (most are double-spaced, but "CodigoReservar S=Salir" is
/// single-spaced), so this scans word by word instead of splitting on
/// spacing: any word containing "=" starts a new option, and every word
/// after it (until the next "="-word) is appended to that option's label.
fn scrape_equals_options(raw: &str) -> Vec<MenuOption> {
    let mut options = Vec::new();
    for line in raw.lines() {
        let mut current: Option<(&str, Vec<&str>)> = None;
        for word in line.split_whitespace() {
            if let Some((key, rest)) = word.split_once('=') {
                if !key.is_empty() {
                    if let Some((key, words)) = current.take() {
                        push_equals_option(&mut options, key, words);
                    }
                    current = Some((key, if rest.is_empty() { vec![] } else { vec![rest] }));
                    continue;
                }
            }
            if let Some((_, words)) = current.as_mut() {
                words.push(word);
            }
        }
        if let Some((key, words)) = current.take() {
            push_equals_option(&mut options, key, words);
        }
    }
    options
}

fn push_equals_option(options: &mut Vec<MenuOption>, key: &str, words: Vec<&str>) {
    let label = words.join(" ");
    let label = label.trim_end_matches(',').trim();
    if !label.is_empty() {
        options.push(MenuOption {
            key: key.to_string(),
            label: label.to_string(),
        });
    }
}

fn course_pattern() -> &'static Regex {
    static PATTERN: OnceLock<Regex> = OnceLock::new();
    // Matches filled schedule rows like " 1.  INSO 4101      080     3    S".
    // Empty slots ("7.", "8.", ...) simply don't match, so they're
    // naturally excluded rather than needing separate handling. Not
    // anchored to end-of-line: a course-selection sub-flow (e.g.
    // `alta_seccion.txt`) overlays a "SECCIONES DISPONIBLES ..." side
    // panel on the same rows as the course list, so trailing content after
    // the status column must be tolerated rather than required absent.
    PATTERN.get_or_init(|| {
        Regex::new(r"(?m)^\s*(\d{1,2})\.\s+([A-Z]{2,4}\s+\d{3,4})\s+(\d{2,3})\s+(\d{1,2})\s+(\S+)")
            .unwrap()
    })
}

fn scrape_courses(raw: &str) -> Vec<ScheduleCourse> {
    course_pattern()
        .captures_iter(raw)
        .map(|c| ScheduleCourse {
            slot: c[1].to_string(),
            course: c[2].split_whitespace().collect::<Vec<_>>().join(" "),
            section: c[3].to_string(),
            credits: c[4].to_string(),
            status: c[5].to_string(),
        })
        .collect()
}

fn bracketed_notice_pattern() -> &'static Regex {
    static PATTERN: OnceLock<Regex> = OnceLock::new();
    // A line that is *entirely* a "<< message >>" advisory, e.g. "<< NO
    // oprimir tecla <Enter> al entrar los datos >>" (note the harmless
    // nested single-bracket "<Enter>") or "<< Esta factura NO ES
    // OFICIAL... >>". Anchored to the whole (trimmed) line so it doesn't
    // match a bracketed key hint embedded mid-instruction, like "Oprima
    // <<Enter>> para Continuar". The `[^<>]` boundaries on the captured
    // content require the brackets to be *exactly* double -- distinct from
    // the remote's other, unrelated "<<<  Oprima Return  >>>" triple-angle
    // style (a plain "press enter" prompt, not an advisory), which would
    // otherwise partially match and capture a mangled leftover bracket.
    PATTERN.get_or_init(|| Regex::new(r"^<<([^<>].*?[^<>])>>$").unwrap())
}

/// Extracts a "<< message >>" advisory line, if the screen has one.
/// Deliberately excludes bare key-name hints like a lone "<<Enter>>" line
/// (real advisories are full phrases, not a single word) -- see
/// `bracketed_notice_pattern`.
fn extract_bracketed_notice(raw: &str) -> Option<String> {
    raw.lines().find_map(|line| {
        let captures = bracketed_notice_pattern().captures(line.trim())?;
        let message = captures[1].trim();
        message.contains(' ').then(|| message.to_string())
    })
}

fn login_fields() -> Vec<LoginField> {
    vec![
        LoginField {
            key: "id_number".to_string(),
            label: "Número de Identificación".to_string(),
            hint: "Ej. 802999999".to_string(),
        },
        LoginField {
            key: "access_code".to_string(),
            label: "Código de Acceso Permanente".to_string(),
            hint: "Ej. 1234".to_string(),
        },
        LoginField {
            key: "ssn_last4".to_string(),
            label: "Seguro Social (últimos 4)".to_string(),
            hint: "Ej. 1234".to_string(),
        },
        LoginField {
            key: "birth_date".to_string(),
            label: "Fecha de Nacimiento".to_string(),
            hint: "Ej. MMDDAAAA".to_string(),
        },
    ]
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

    // "Ej. 802999999" is unique to the ID-number field's hint and doesn't
    // depend on any accented character, unlike this screen's labels (which
    // arrive mangled -- see `login_fields`'s doc comment).
    if raw.contains("Ej. 802999999") {
        return TuiScreen::Login {
            fields: login_fields(),
        };
    }

    if raw.contains("Indique Semestre") {
        return TuiScreen::SelectPeriod {
            options: scrape_equals_options(raw),
        };
    }

    if raw.contains("M A T R I C U L A") {
        let courses = scrape_courses(raw);
        let prompt = if raw.contains("[Bajas]") {
            MatriculaPrompt::Bajas
        } else if raw.contains("[Altas]") {
            MatriculaPrompt::Altas
        } else if raw.contains("[Cambio]") {
            MatriculaPrompt::Cambio
        } else {
            MatriculaPrompt::Actions {
                options: scrape_equals_options(raw),
            }
        };
        return TuiScreen::Matricula { courses, prompt };
    }

    if raw.contains("MENU PRINCIPAL") {
        return TuiScreen::MainMenu {
            options: scrape_options(raw),
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
    if let Some(message) = extract_bracketed_notice(raw) {
        return TuiScreen::Notice {
            message,
            raw: raw.to_string(),
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

    // Read directly from the real transcripts the user captured with a real
    // account (screens/*.txt at the repo root), not retyped -- so these
    // tests fail loudly if the real formatting ever turns out to differ
    // from what's assumed here.
    const LOGIN: &str = include_str!("../../screens/login.txt");
    const SELECT_PERIOD: &str = include_str!("../../screens/matricula/select_period.txt");
    const SELECT: &str = include_str!("../../screens/matricula/select.txt");
    const BAJAS: &str = include_str!("../../screens/matricula/bajas.txt");

    #[test]
    fn classifies_login_form() {
        let TuiScreen::Login { fields } = classify(LOGIN) else {
            panic!("expected Login");
        };
        assert_eq!(
            fields.iter().map(|f| f.key.as_str()).collect::<Vec<_>>(),
            ["id_number", "access_code", "ssn_last4", "birth_date"]
        );
    }

    #[test]
    fn classifies_select_period() {
        let TuiScreen::SelectPeriod { options } = classify(SELECT_PERIOD) else {
            panic!("expected SelectPeriod");
        };
        assert_eq!(
            options,
            vec![
                MenuOption { key: "1".into(), label: "1er Sem".into() },
                MenuOption { key: "2".into(), label: "2do Sem".into() },
                MenuOption {
                    key: "3".into(),
                    label: "1er Verano o Verano Extendido".into()
                },
                MenuOption {
                    key: "4".into(),
                    label: "2do Verano o Admision Temprana".into()
                },
                MenuOption { key: "S".into(), label: "salir".into() },
            ]
        );
    }

    #[test]
    fn classifies_matricula_with_actions_prompt() {
        let TuiScreen::Matricula { courses, prompt } = classify(SELECT) else {
            panic!("expected Matricula");
        };
        assert_eq!(
            courses,
            vec![
                ScheduleCourse {
                    slot: "1".into(),
                    course: "INGE 3045".into(),
                    section: "086".into(),
                    credits: "3".into(),
                    status: "S".into(),
                },
                ScheduleCourse {
                    slot: "2".into(),
                    course: "BIOL 3031".into(),
                    section: "066".into(),
                    credits: "3".into(),
                    status: "S".into(),
                },
                ScheduleCourse {
                    slot: "3".into(),
                    course: "EDFI 3645".into(),
                    section: "041".into(),
                    credits: "2".into(),
                    status: "S".into(),
                },
                ScheduleCourse {
                    slot: "4".into(),
                    course: "FILO 4045".into(),
                    section: "020".into(),
                    credits: "3".into(),
                    status: "S".into(),
                },
            ]
        );
        let MatriculaPrompt::Actions { options } = prompt else {
            panic!("expected Actions prompt");
        };
        assert_eq!(
            options.iter().map(|o| o.key.as_str()).collect::<Vec<_>>(),
            ["A", "B", "C", "H", "P", "M", "F", "O", "S"]
        );
        assert_eq!(options[0].label, "Alta");
        // The single-space-separated tail ("...O=CodigoReservar S=Salir")
        // is the case that broke a whitespace-run-based split.
        assert_eq!(options[7].label, "CodigoReservar");
        assert_eq!(options[8].label, "Salir");
    }

    #[test]
    fn classifies_matricula_with_bajas_prompt() {
        let TuiScreen::Matricula { courses, prompt } = classify(BAJAS) else {
            panic!("expected Matricula");
        };
        assert_eq!(courses.len(), 4);
        assert_eq!(prompt, MatriculaPrompt::Bajas);
    }

    const ALTA: &str = include_str!("../../screens/matricula/alta.txt");
    const CAMBIOS: &str = include_str!("../../screens/matricula/cambios.txt");
    const ALTA_SECCION: &str = include_str!("../../screens/matricula/alta_seccion.txt");
    const EVALUO_PAGO: &str = include_str!("../../screens/matricula/evaluo_pago.txt");
    const HORARIOS_DE_SECCION: &str =
        include_str!("../../screens/matricula/horarios_de_seccion.txt");

    #[test]
    fn classifies_matricula_with_altas_prompt() {
        let TuiScreen::Matricula { courses, prompt } = classify(ALTA) else {
            panic!("expected Matricula");
        };
        assert_eq!(courses.len(), 4);
        assert_eq!(prompt, MatriculaPrompt::Altas);
    }

    #[test]
    fn classifies_matricula_with_cambio_prompt() {
        let TuiScreen::Matricula { courses, prompt } = classify(CAMBIOS) else {
            panic!("expected Matricula");
        };
        assert_eq!(courses.len(), 4);
        assert_eq!(prompt, MatriculaPrompt::Cambio);
    }

    #[test]
    fn course_rows_tolerate_a_trailing_side_panel() {
        // alta_seccion.txt overlays "SECCIONES DISPONIBLES CURSO: ..." text
        // after the status column on some rows -- course_pattern must not
        // require end-of-line right after the status field.
        let TuiScreen::Matricula { courses, .. } = classify(ALTA_SECCION) else {
            panic!("expected Matricula");
        };
        assert_eq!(courses.len(), 4);
        assert_eq!(courses[0].course, "INGE 3045");
        assert_eq!(courses[0].status, "S");
    }

    #[test]
    fn bracketed_notice_on_an_unmodeled_screen_is_extracted_cleanly() {
        // evaluo_pago.txt (the payment/invoice screen) isn't specifically
        // modeled, but its "<< ... >>" disclaimer should still be pulled
        // out as a Notice message rather than left buried in raw text.
        let TuiScreen::Notice { message, .. } = classify(EVALUO_PAGO) else {
            panic!("expected Notice");
        };
        assert_eq!(
            message,
            "Esta factura NO ES OFICIAL, cargos y pagos sujetos a verificacion"
        );
        // Not a "no esta disponible" rejection, so it must not become a
        // command error.
        assert!(TuiScreen::Notice {
            message,
            raw: String::new()
        }
        .or_err()
        .is_ok());
    }

    #[test]
    fn triple_angle_bracket_prompt_is_not_mistaken_for_an_advisory() {
        // horarios_de_seccion.txt ends with "<<<  Oprima Return  >>>" --
        // triple angle brackets, a plain "press enter" prompt, not a "<<
        // message >>" advisory. It isn't a modeled screen either, so this
        // should land in Unknown with the raw text intact, not a mangled
        // Notice with leftover bracket characters.
        assert!(matches!(
            classify(HORARIOS_DE_SECCION),
            TuiScreen::Unknown { .. }
        ));
    }

    #[test]
    fn extract_bracketed_notice_rejects_bare_key_hints() {
        assert_eq!(extract_bracketed_notice("<<Enter>>"), None);
        assert_eq!(
            extract_bracketed_notice("Oprima <<Enter>> para Continuar"),
            None
        );
        assert_eq!(
            extract_bracketed_notice("<<<  Oprima Return  >>>"),
            None
        );
    }
}
