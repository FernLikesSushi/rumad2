//! Turns the raw VT100 screen text from a `TuiSession` into a typed
//! `ClassifiedScreen` the frontend can render as native UI instead of a
//! terminal: always a `TuiScreen` (freshly re-derived from the current raw
//! text every time), plus an optional `Dialog` overlaid on top of it.
//!
//! Every real student connects over SSH as the same shared `estudiante`
//! account (that's transport-layer only -- SSH auth is *not* the real
//! authentication). The demo/no-real-account case drops straight into
//! `MENU PRINCIPAL` and answers account-specific options (like "Seleccion
//! de Secciones") with "Cuenta ESTUDIANTE NO esta disponible por el
//! momento" -- a temporary service-window/maintenance rejection
//! (`ClassifiedScreen::or_err` promotes it to a command error) rather than
//! ever reaching the real per-student flow. With a real account, selecting
//! "Seleccion de Secciones" instead leads to: `Login` (a 4-field
//! ID/PIN/SSN-last-4/birth-date form -- the *actual* authentication,
//! separate from SSH) -> `Menu(SelectPeriod)` (which semester) ->
//! `Matricula` (the student's course schedule, with an action menu and
//! `Altas`/`Bajas` add/drop-course sub-modes). `MENU PRINCIPAL`'s option 5
//! ("Ver otra informacion") leads to a separate submenu,
//! `Menu(MenuDespliegue)` ("MENU DESPLIEGUE"), with its own read-only
//! screens: option 5 ("Turno de seleccion...") is a static redirect
//! notice (`turno_seleccion.txt`, caught by
//! `notice::extract_bracketed_or_boxed_notice`'s boxed case, not a
//! dedicated variant -- there's nothing else on that screen), and option 6
//! ("Horario de cursos disponibles en Matricula") is `Menu(HorarioSemester)`
//! (pick a semester) -> `Search(HorarioCurso)` (search a course code) ->
//! `Search(HorarioSeccion)` (narrow a multi-section course down to one) ->
//! `CourseResults` (the results -- also reachable from `WeeklySchedule`'s
//! sibling option 8, "Horario de matricula grafico", the student's own
//! schedule as a day/time grid) -- all distinct from `Matricula`'s
//! interactive course list. Grounded in real transcripts captured by the
//! user with a real account: `screens/login.txt`, `screens/matricula/*.txt`,
//! and `screens/menu_despliegue/*.txt` at the repo root (that directory
//! can gain new files or be reorganized as the user captures more --
//! re-check paths rather than assuming).
//!
//! **Dialogs overlay screens, they aren't screens themselves.** The
//! remote sometimes shows a message *alongside* whatever's already
//! rendering -- a rejection ("no esta disponible", "*** message ***"), an
//! informational aside ("<< message >>", or a boxed "****...****"
//! variant), or the "Programa en Proceso" marquee while it computes a
//! result asynchronously -- without replacing that screen's own content,
//! which stays fully intact and classifiable underneath. `classify` models
//! this literally: `detect_dialog` and `classify_screen` each scan the
//! *same* raw text independently and their results are combined into one
//! `ClassifiedScreen { screen, dialog }`, rather than the old design where
//! a detected notice would short-circuit and *replace* the screen
//! (forcing the frontend to separately track "the last real screen" to
//! paper over that). `Disconnected` is *not* derived here at all --
//! its "PROCESO CONCLUIDO" banner text can show up without the SSH
//! channel actually closing, so it's just another bracketed advisory as
//! far as this module is concerned; `commands::exec::handle_scene_change`
//! is what actually decides `Disconnected`, from the live channel's own
//! EOF state.
//!
//! **Identical screens share one struct.** Four real prompts are just a
//! numbered/lettered list of options (`MenuScreen`, distinguished by
//! `MenuKind`); two are an identical bare free-text search with no data of
//! their own (`SearchScreen`, distinguished by `SearchKind`). Screens that
//! only *look* similar but aren't -- `CourseResults`/`WeeklySchedule`
//! being different table shapes, `Login`'s fixed-field form -- stay their
//! own dedicated types instead of being forced into a shared shape.
//!
//! Each screen type that has dedicated data gets its own submodule for its
//! types/scrapers/tests/`impl RumadScreen` (`interact.rs` owns only the
//! shared trait definition, not any screen's implementation of it --
//! `UnknownScreen`'s lives here instead, since it has no submodule of its
//! own); `classify`/`classify_screen`/`detect_dialog` themselves stay in
//! this file rather than being scattered, since they need every
//! submodule's detection hint in one place to reason about.

mod confirmed_schedule;
mod course_results;
mod interact;
mod login;
mod matricula;
mod menu;
mod notice;
mod scrape;
mod search;
mod weekly_schedule;

use serde::Serialize;

// Re-exported (rather than just `pub`) at this flat `screens::` path since
// `screens` is itself `pub` (see `lib.rs`) for use as a scraping library
// independent of the Tauri app -- these submodules stay private so callers
// go through this curated surface instead of the internal module layout.
// Field types like `CourseSection` are included even though nothing inside
// this crate names them directly: an external caller needs them to do
// anything with e.g. `CourseResultsScreen.sections`.
pub use confirmed_schedule::{ConfirmedCourse, ConfirmedScheduleScreen};
pub use course_results::{CourseResultsScreen, CourseSection};
pub use interact::RumadScreen;
pub use login::{LoginField, LoginScreen};
pub use matricula::{MatriculaMode, MatriculaScreen, ScheduleCourse};
pub use menu::{MenuKind, MenuScreen};
pub use search::{SearchKind, SearchScreen};
pub use weekly_schedule::{ScheduleRow, WeeklyScheduleScreen};

/// One `key. label` (or `key=label`) entry scraped off a `MenuScreen`,
/// e.g. `{key: "2", label: "Seleccion de Secciones  (Matricula)"}`.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct MenuOption {
    pub key: String,
    pub label: String,
}

/// Anything not yet recognized, e.g. `horario_confirmado.txt` or
/// `reserva.txt` (real captured screens with no dedicated variant yet).
/// `options` is a best-effort scrape of numbered lines ("N.  LABEL"), so
/// unmodeled screens are still navigable from the frontend rather than
/// dead ends.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct UnknownScreen {
    pub raw: String,
    pub options: Vec<MenuOption>,
}

/// No dedicated interaction of its own -- inherits every `RumadScreen`
/// default (`select`/`line` for the best-effort scraped `options`, "0" to
/// exit, `can_exit` true).
#[async_trait::async_trait]
impl RumadScreen for UnknownScreen {}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "kind")]
pub enum TuiScreen {
    /// A numbered/lettered menu, e.g. `MENU PRINCIPAL`'s "1.  Seleccion de
    /// Secciones  (Matricula)" ... "0.  SALIR DEL SISTEMA" -- see
    /// `MenuKind` for the four real prompts this covers and how they
    /// differ.
    Menu(MenuScreen),
    /// The real per-student authentication form (ID number, permanent
    /// access code, last 4 of SSN, birth date) shown after selecting
    /// "Seleccion de Secciones" -- separate from and unrelated to the SSH
    /// login, which every student shares (`estudiante@...`). The remote
    /// warns not to press Enter while filling it in; use `send_text`
    /// (no trailing Enter), not `send_input`.
    Login(LoginScreen),
    /// The student's course schedule (`M A T R I C U L A`) plus whichever
    /// sub-mode is currently active, e.g. a course table alongside an
    /// `Actions` prompt ("A=Alta B=Baja C=Cambio ... S=Salir").
    Matricula(MatriculaScreen),
    /// Read-only search results for one course code (`MENU DESPLIEGUE` ->
    /// "Horario de cursos disponibles en Matricula"), e.g. "C u r s o:
    /// HIST 3220" with a table of sections underneath (room/schedule/
    /// professor/capacity). Not the student's own schedule, just a
    /// lookup.
    CourseResults(CourseResultsScreen),
    /// The student's confirmed enrollment (`Matricula`'s own
    /// "[CONFIRMADA]" report): one row per enrolled course with its
    /// section/credits/room/schedule/professor. Distinct from
    /// `CourseResults` (every section of one course, a lookup) and
    /// `Matricula` (the interactive course list, no room/schedule/
    /// professor data).
    ConfirmedSchedule(ConfirmedScheduleScreen),
    /// The student's own schedule as a weekly day/time grid (`MENU
    /// DESPLIEGUE` -> "Horario de matricula grafico") -- purely
    /// informational, unlike `Matricula`'s interactive course list.
    WeeklySchedule(WeeklyScheduleScreen),
    /// A bare free-text search prompt, e.g. "C u r s o  (Ej. QUIM3001L)
    /// Puede indicar solo MATERIA             [PF4=(9)Fin]" -- see
    /// `SearchKind` for the two real prompts this covers.
    Search(SearchScreen),
    /// The SSH channel has actually closed (`TuiSession::is_closed()`),
    /// meaning there's nothing left to interact with -- decided by the
    /// command layer from the live channel's EOF state, not by this
    /// module. Not the same thing as seeing the remote's own "<<< P R O
    /// C E S O  C O N C L U I D O >>>" banner text: that can show up
    /// without the channel actually closing, so it's just an ordinary
    /// bracketed `Dialog::Notice` as far as `classify` is concerned.
    Disconnected,
    Unknown(UnknownScreen),
}

impl TuiScreen {
    /// This screen's `RumadScreen` impl, or `None` for `Disconnected` --
    /// no live session left to send anything to. `pub`, same reason as
    /// the `RumadScreen` re-export above.
    pub fn as_rumad_screen(&self) -> Option<&dyn RumadScreen> {
        match self {
            TuiScreen::Menu(s) => Some(s),
            TuiScreen::Login(s) => Some(s),
            TuiScreen::Matricula(s) => Some(s),
            TuiScreen::CourseResults(s) => Some(s),
            TuiScreen::ConfirmedSchedule(s) => Some(s),
            TuiScreen::WeeklySchedule(s) => Some(s),
            TuiScreen::Search(s) => Some(s),
            TuiScreen::Unknown(s) => Some(s),
            TuiScreen::Disconnected => None,
        }
    }
}

/// A message shown *alongside* whatever `TuiScreen` is currently
/// rendering, rather than replacing it -- see this module's doc comment.
/// The frontend overlays these (a dialog, a banner) instead of switching
/// what screen it's showing.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "kind")]
pub enum Dialog {
    /// A one-line rejection or informational message, e.g. "Cuenta
    /// ESTUDIANTE NO esta disponible por el momento" or "*** Curso NO
    /// Existe en Archivo MTR-HORARIO ***". Not every `Notice` warrants
    /// treating the action as failed -- see `ClassifiedScreen::or_err` for
    /// the one pattern known to mean that today.
    Notice { message: String, raw: String },
    /// The remote's own "<<<  Programa  en  Proceso  >>>" marquee, shown
    /// while it's still computing a result asynchronously and then
    /// redraws again on its own -- not specific to any one screen.
    /// `commands::exec::spawn_screen_watcher` keeps polling in the
    /// background and pushes the eventual real result to the frontend as
    /// a `screen-changed` event, instead of the frontend needing to poll
    /// for it.
    Processing,
}

/// The result of classifying one raw screen: always a `TuiScreen`
/// (freshly re-derived from the current raw text, not "whatever it was
/// before"), plus an optional `Dialog` overlaid on top of it.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClassifiedScreen {
    pub screen: TuiScreen,
    pub dialog: Option<Dialog>,
    /// Whether `screen` has a meaningful `RumadScreen::exit()` to reach --
    /// see that method's own doc comment for exactly what counts. The
    /// frontend uses this to decide whether to render its one shared exit
    /// control, instead of each screen component hardcoding its own copy
    /// of that decision.
    pub can_exit: bool,
    /// Same idea as `can_exit`, for `RumadScreen::continue_screen()`.
    pub can_continue: bool,
}

impl ClassifiedScreen {
    /// Turns a rejection meaning "the service you tried to reach is down
    /// right now" into an `Err`, so the command layer doesn't hand the
    /// frontend a dead-end screen to render. Any other `Dialog`, including
    /// other `Notice` messages, passes through unchanged.
    pub fn or_err(self) -> Result<ClassifiedScreen, String> {
        if let Some(Dialog::Notice { message, .. }) = &self.dialog {
            let lower = message.to_lowercase();
            if lower.contains("no esta disponible") || lower.contains("no está disponible") {
                return Err(message.clone());
            }
        }
        Ok(self)
    }
}

pub fn classify(raw: &str) -> ClassifiedScreen {
    // `TuiScreen::Disconnected` is deliberately *not* derived here from the
    // "PROCESO CONCLUIDO" banner text -- that text can apparently show up
    // without the SSH channel actually closing, so treating it as an
    // unconditional graceful-end-of-session signal was wrong. The banner
    // is real remote output, though, so `detect_dialog` below still
    // surfaces it as an ordinary `Dialog::Notice` (its own dedicated
    // collapsed-text check, same as "Programa en Proceso"). Whether the
    // session is *actually* over is a live-channel fact, not a text fact
    // -- see `commands::exec::handle_scene_change`, which checks
    // `TuiSession::is_closed()` and overrides to `Disconnected` only when
    // the channel has genuinely reached EOF.
    let collapsed: String = raw.chars().filter(|c| !c.is_whitespace()).collect();
    let screen = classify_screen(raw);
    let can_exit = screen.as_rumad_screen().is_some_and(|s| s.can_exit());
    let can_continue = screen.as_rumad_screen().is_some_and(|s| s.can_continue());
    ClassifiedScreen {
        screen,
        dialog: detect_dialog(raw, &collapsed),
        can_exit,
        can_continue,
    }
}

/// The screen underneath, regardless of whether a `Dialog` is also
/// overlaid on it -- see this module's doc comment. None of the checks
/// below share any detection text with each other (each screen's own
/// submodule documents why its hint is unique), so unlike the old design,
/// their relative order doesn't matter for correctness.
fn classify_screen(raw: &str) -> TuiScreen {
    if raw.contains(login::DETECT_HINT) {
        return TuiScreen::Login(LoginScreen {
            fields: login::login_fields(),
        });
    }

    if raw.contains("Indique Semestre") {
        return TuiScreen::Menu(MenuScreen {
            menu: MenuKind::SelectPeriod,
            options: scrape::scrape_equals_options(raw),
        });
    }

    if raw.contains("M A T R I C U L A") {
        let courses = matricula::scrape_courses(raw);
        let mode = if raw.contains("[Bajas]") {
            MatriculaMode::Bajas
        } else if raw.contains("[Altas]") {
            MatriculaMode::Altas
        } else if raw.contains("[Cambio]") {
            MatriculaMode::Cambio
        } else {
            MatriculaMode::Actions {
                options: scrape::scrape_equals_options(raw),
            }
        };
        return TuiScreen::Matricula(MatriculaScreen { courses, mode });
    }

    if raw.contains("MENU PRINCIPAL") {
        return TuiScreen::Menu(MenuScreen {
            menu: MenuKind::MainMenu,
            options: scrape::scrape_options(raw),
        });
    }

    if raw.contains("MENU DESPLIEGUE") {
        return TuiScreen::Menu(MenuScreen {
            menu: MenuKind::MenuDespliegue,
            options: scrape::scrape_options(raw),
        });
    }

    if let Some((course_code, course_title)) = course_results::detect_course(raw) {
        return TuiScreen::CourseResults(CourseResultsScreen {
            course_code,
            course_title,
            sections: course_results::scrape_sections(raw),
        });
    }

    if raw.contains(confirmed_schedule::DETECT_HINT) {
        return TuiScreen::ConfirmedSchedule(confirmed_schedule::scrape(raw));
    }

    if weekly_schedule::detect(raw) {
        return TuiScreen::WeeklySchedule(weekly_schedule::scrape(raw));
    }

    if raw.contains(search::HORARIO_CURSO_DETECT_HINT) {
        return TuiScreen::Search(SearchScreen {
            search: SearchKind::HorarioCurso,
        });
    }

    if raw.contains(search::HORARIO_SECCION_DETECT_HINT) {
        return TuiScreen::Search(SearchScreen {
            search: SearchKind::HorarioSeccion,
        });
    }

    if raw.contains(menu::HORARIO_SEMESTER_DETECT_HINT) {
        return TuiScreen::Menu(MenuScreen {
            menu: MenuKind::HorarioSemester,
            options: menu::scrape_horario_semester_options(raw),
        });
    }

    TuiScreen::Unknown(UnknownScreen {
        raw: raw.to_string(),
        options: scrape::scrape_options(raw),
    })
}

/// The `Dialog` overlaid on top of whatever `classify_screen` finds, if
/// any -- checked in priority order since more than one pattern could in
/// principle match the same raw text (e.g. a rejection could theoretically
/// also happen to read as a boxed advisory): a specific "no esta
/// disponible" rejection first (needs to win because `or_err` keys off its
/// exact wording), then the generalized "*** message ***" rejection
/// pattern, then the "Programa en Proceso" marquee, then the lowest-
/// priority informational advisories ("<< message >>" or a boxed
/// "****...****" variant).
fn detect_dialog(raw: &str, collapsed: &str) -> Option<Dialog> {
    let notice_line = raw.lines().find(|line| {
        let lower = line.to_lowercase();
        lower.contains("no esta disponible") || lower.contains("no está disponible")
    });
    if let Some(line) = notice_line {
        return Some(Dialog::Notice {
            message: line.trim().to_string(),
            raw: raw.to_string(),
        });
    }

    if let Some(message) = notice::extract_starred_notice(raw) {
        return Some(Dialog::Notice {
            message,
            raw: raw.to_string(),
        });
    }

    // Pads every letter with spaces for a marquee effect too ("<<<
    // Programa  en  Proceso  >>>"). Triple-angle-bracket, like "PROCESO
    // CONCLUIDO" below -- `notice::extract_bracketed_or_boxed_notice`'s
    // regex only matches *exactly* double brackets (see its own doc
    // comment), so this needs its own collapsed-text check rather than
    // falling through to that.
    if collapsed.contains("ProgramaenProceso") {
        return Some(Dialog::Processing);
    }

    // Same triple-bracket reasoning as "Programa en Proceso" above --
    // shown as a `Notice` rather than silently dropped, even though
    // (per `TuiScreen::Disconnected`'s doc comment) this text alone no
    // longer means the session actually ended.
    if collapsed.contains("PROCESOCONCLUIDO") {
        return Some(Dialog::Notice {
            message: "PROCESO CONCLUIDO".to_string(),
            raw: raw.to_string(),
        });
    }

    if let Some(message) = notice::extract_bracketed_or_boxed_notice(raw) {
        return Some(Dialog::Notice {
            message,
            raw: raw.to_string(),
        });
    }

    None
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
        let TuiScreen::Menu(MenuScreen { menu, options }) = classify(MAIN_MENU).screen else {
            panic!("expected Menu");
        };
        assert_eq!(menu, MenuKind::MainMenu);
        assert_eq!(options.len(), 7);
        assert_eq!(options[0].key, "1");
        assert_eq!(options[1].label, "Seleccion de Secciones  (Matricula)");
        assert_eq!(options.last().unwrap().key, "0");
        assert_eq!(options.last().unwrap().label, "SALIR DEL SISTEMA");
    }

    #[test]
    fn classifies_account_unavailable_notice() {
        let raw = "              Cuenta ESTUDIANTE NO esta disponible por el momento";
        let result = classify(raw);
        assert_eq!(
            result.dialog,
            Some(Dialog::Notice {
                message: "Cuenta ESTUDIANTE NO esta disponible por el momento".to_string(),
                raw: raw.to_string(),
            })
        );
        assert_eq!(
            result.or_err(),
            Err("Cuenta ESTUDIANTE NO esta disponible por el momento".to_string())
        );
    }

    #[test]
    fn main_menu_passes_through_or_err() {
        assert!(classify(MAIN_MENU).or_err().is_ok());
    }

    #[test]
    fn notice_and_screen_come_through_together_when_menu_principal_is_also_on_screen() {
        // Grounded scenario: selecting an unavailable option redisplays
        // MENU PRINCIPAL with the rejection notice still on screen (its
        // own row, per vt100's one-line-per-terminal-row rendering)
        // rather than replacing it entirely -- both the dialog and the
        // still-classifiable menu underneath must come through.
        let raw = format!(
            "              Cuenta ESTUDIANTE NO esta disponible por el momento\n{MAIN_MENU}"
        );
        let result = classify(&raw);
        assert!(matches!(
            result.screen,
            TuiScreen::Menu(MenuScreen {
                menu: MenuKind::MainMenu,
                ..
            })
        ));
        assert_eq!(
            result.or_err(),
            Err("Cuenta ESTUDIANTE NO esta disponible por el momento".to_string())
        );
    }

    #[test]
    fn proceso_concluido_banner_does_not_force_disconnected() {
        // The banner text alone isn't authoritative for `Disconnected`
        // (see `TuiScreen::Disconnected`'s doc comment) -- only
        // `commands::exec::handle_scene_change`'s live `TuiSession::
        // is_closed()` check is, which this text-only classifier has no
        // access to. So this just overlays as an ordinary `Notice`.
        let raw = "\
UNIVERSIDAD DE PUERTO RICO
                      Recinto Universitario de Mayaguez
18/ago/2026                                                            10:43 am









                <<<   P R O C E S O    C O N C L U I D O   >>>
";
        let result = classify(raw);
        assert!(!matches!(result.screen, TuiScreen::Disconnected));
        assert_eq!(
            result.dialog,
            Some(Dialog::Notice {
                message: "PROCESO CONCLUIDO".to_string(),
                raw: raw.to_string(),
            })
        );
    }

    #[test]
    fn processing_marker_overlays_main_menu_underneath_it() {
        // Not specific to the course/section-search flow that surfaced
        // it -- any screen doing real work server-side can show this, so
        // the underlying screen must still classify normally underneath
        // it (same reasoning as the notice case above).
        let raw = format!("                       <<<  Programa  en  Proceso  >>>\n{MAIN_MENU}");
        let result = classify(&raw);
        assert!(matches!(
            result.screen,
            TuiScreen::Menu(MenuScreen {
                menu: MenuKind::MainMenu,
                ..
            })
        ));
        assert_eq!(result.dialog, Some(Dialog::Processing));
    }

    // `RESERVA` is a real (redacted) transcript captured for a flow that
    // isn't specifically modeled yet -- per this module's own doc
    // comment, `TuiScreen` variants only get written against a real
    // transcript, never guessed ahead of one being captured. That test
    // locks in that `classify` still handles it gracefully (no panic,
    // `Unknown` with the raw text preserved) rather than misfiring into
    // an unrelated variant on some incidental substring match.
    // `horario_confirmado.txt` used to sit alongside it here too, but is
    // now modeled as `ConfirmedSchedule` (see `confirmed_schedule.rs`'s
    // own classification test). Everything else below *is* modeled now
    // (each owns its own classification tests in its own submodule) but
    // stays listed here for the shared "not MainMenu" cross-check.
    const RESERVA: &str = include_str!("../../../screens/matricula/reserva.txt");
    const MENU_DESPLIEGUE: &str = include_str!("../../../screens/menu_despliegue/menu.txt");
    const HORARIO_SEMESTRE: &str =
        include_str!("../../../screens/menu_despliegue/horario_semestre.txt");
    const HORARIO_CURSO: &str = include_str!("../../../screens/menu_despliegue/horario_curso.txt");
    const HORARIO_PROCESANDO: &str =
        include_str!("../../../screens/menu_despliegue/horario_procesando.txt");
    const HORARIO_RESULTADOS_MULTI: &str =
        include_str!("../../../screens/menu_despliegue/horario_resultados_multi.txt");
    const HORARIO_RESULTADOS_SIMPLE: &str =
        include_str!("../../../screens/menu_despliegue/horario_resultados_simple.txt");
    const TURNO_SELECCION: &str =
        include_str!("../../../screens/menu_despliegue/turno_seleccion.txt");

    #[test]
    fn unmodeled_matricula_screens_fall_back_to_unknown() {
        let TuiScreen::Unknown(UnknownScreen { raw: got, .. }) = classify(RESERVA).screen else {
            panic!("expected Unknown for:\n{RESERVA}");
        };
        assert_eq!(got, RESERVA);
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
                !matches!(
                    classify(raw).screen,
                    TuiScreen::Menu(MenuScreen {
                        menu: MenuKind::MainMenu,
                        ..
                    })
                ),
                "expected not MainMenu for:\n{raw}"
            );
        }
    }
}
