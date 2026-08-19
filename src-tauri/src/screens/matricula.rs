//! The student's course schedule (`M A T R I C U L A`) plus whichever
//! sub-mode is currently active.

use regex::Regex;
use serde::Serialize;
use std::sync::OnceLock;

use super::{MenuOption, RumadScreen};
use crate::ssh::session::TuiSession;

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

/// Which sub-mode `Matricula` is currently showing. The header and
/// course list stay identical across these -- only the bottom prompt (and
/// what input it expects) changes.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "kind")]
pub enum MatriculaMode {
    /// When the student first enters `Matricula`, we have actions to invoke.
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

/// The student's course schedule, e.g. rows like "1.  INSO 4101      080
/// 3    S" under a header reading "M A T R I C U L A", plus whichever
/// `MatriculaMode` sub-prompt is currently active underneath it.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct MatriculaScreen {
    pub courses: Vec<ScheduleCourse>,
    pub mode: MatriculaMode,
}

impl RumadScreen for MatriculaScreen {
    /// `Actions` exits via its own `S`=Salir option -- a normal `select`,
    /// same as `MenuScreen`'s "0"/"S=salir" (see that impl's `exit` doc
    /// comment for why that still counts as a real exit destination, not
    /// a bail). `Bajas`/`Altas`/`Cambio` have no such listed option --
    /// they exit via the free-text "FIN" instead.
    fn exit(&self, session: &mut TuiSession) -> anyhow::Result<()> {
        match self.mode {
            MatriculaMode::Actions { .. } => self.select(session, "S"),
            MatriculaMode::Bajas | MatriculaMode::Altas | MatriculaMode::Cambio => {
                self.line(session, "FIN")
            }
        }
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

pub(super) fn scrape_courses(raw: &str) -> Vec<ScheduleCourse> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::screens::{classify, MatriculaScreen, TuiScreen};

    const SELECT: &str = include_str!("../../../screens/matricula/select.txt");
    const BAJAS: &str = include_str!("../../../screens/matricula/bajas.txt");
    const ALTA: &str = include_str!("../../../screens/matricula/alta.txt");
    const CAMBIOS: &str = include_str!("../../../screens/matricula/cambios.txt");
    const ALTA_SECCION: &str = include_str!("../../../screens/matricula/alta_seccion.txt");

    #[test]
    fn classifies_matricula_with_actions_mode() {
        let TuiScreen::Matricula(MatriculaScreen { courses, mode }) = classify(SELECT).screen
        else {
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
        let MatriculaMode::Actions { options } = mode else {
            panic!("expected Actions mode");
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
    fn classifies_matricula_with_bajas_mode() {
        let TuiScreen::Matricula(MatriculaScreen { courses, mode }) = classify(BAJAS).screen else {
            panic!("expected Matricula");
        };
        assert_eq!(courses.len(), 4);
        assert_eq!(mode, MatriculaMode::Bajas);
    }

    #[test]
    fn classifies_matricula_with_altas_mode() {
        let TuiScreen::Matricula(MatriculaScreen { courses, mode }) = classify(ALTA).screen else {
            panic!("expected Matricula");
        };
        assert_eq!(courses.len(), 4);
        assert_eq!(mode, MatriculaMode::Altas);
    }

    #[test]
    fn classifies_matricula_with_cambio_mode() {
        let TuiScreen::Matricula(MatriculaScreen { courses, mode }) = classify(CAMBIOS).screen
        else {
            panic!("expected Matricula");
        };
        assert_eq!(courses.len(), 4);
        assert_eq!(mode, MatriculaMode::Cambio);
    }

    #[test]
    fn course_rows_tolerate_a_trailing_side_panel() {
        // alta_seccion.txt overlays "SECCIONES DISPONIBLES CURSO: ..." text
        // after the status column on some rows -- course_pattern must not
        // require end-of-line right after the status field.
        let TuiScreen::Matricula(MatriculaScreen { courses, .. }) = classify(ALTA_SECCION).screen
        else {
            panic!("expected Matricula");
        };
        assert_eq!(courses.len(), 4);
        assert_eq!(courses[0].course, "INGE 3045");
        assert_eq!(courses[0].status, "S");
    }
}
