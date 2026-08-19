//! `MENU DESPLIEGUE` option 6 ("Horario de cursos disponibles en
//! Matricula"): pick a semester, search a course code, then search a
//! section within it -- the results land on the already-modeled
//! `CourseResultsScreen`. `HorarioSemesterScreen`'s own "Indique semestre"
//! prompt is distinct from `SelectPeriodScreen`'s "Indique Semestre:"
//! (capital S) in the `Login`/`Matricula` flow: this one is lowercase,
//! compact ("1=1erVer" rather than "1=1er Sem"), parenthesized, and exits
//! via `[PF4=(9)Fin]` instead of a listed "S=salir" option (see
//! `screens::tests::horario_semestre_is_not_mistaken_for_select_period`).
//! `HorarioCursoScreen`/`HorarioSeccionScreen` carry no data of their own
//! -- their "(Ej. QUIM3001L)"/"(Ej. 001#)" hints are fixed boilerplate,
//! not per-user data, so they're hardcoded and localized on the frontend
//! (`Messages.horarioCursoHint`/`horarioSeccionHint`) rather than scraped
//! here, same tradeoff as `Login`'s fields and the screen titles.

use regex::Regex;
use serde::Serialize;
use std::sync::OnceLock;

use super::MenuOption;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct HorarioSemesterScreen {
    pub options: Vec<MenuOption>,
}

/// Unique to this prompt: lowercase "semestre", unlike `SelectPeriodScreen`'s
/// "Indique Semestre:".
pub(super) const SEMESTER_DETECT_HINT: &str = "Indique semestre";

fn compact_equals_pattern() -> &'static Regex {
    static PATTERN: OnceLock<Regex> = OnceLock::new();
    // "1=1erVer   2=1erSem   3=2doSem   4=2doVer" -- no spaces around "=".
    // `scrape::scrape_equals_options`'s generic word-splitting would
    // misfire here: the enclosing parens glue onto the adjacent
    // digit/word ("(1=1erVer", "4=2doVer)"), and the line's own
    // "[PF4=(9)Fin]" hint isn't a real option. Anchoring the key to `\d+`
    // specifically (PF4's key is letters) and the label to `\w+` (which
    // stops at the ")" following the last option, and can never start a
    // match right after "PF4=" since a "(" immediately follows there)
    // sidesteps both without needing to special-case the parens at all.
    PATTERN.get_or_init(|| Regex::new(r"(\d+)=(\w+)").unwrap())
}

pub(super) fn scrape_semester_options(raw: &str) -> Vec<MenuOption> {
    compact_equals_pattern()
        .captures_iter(raw)
        .map(|c| MenuOption { key: c[1].to_string(), label: c[2].to_string() })
        .collect()
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct HorarioCursoScreen;

/// "(Ej." is unique to this search prompt -- the results screen
/// (`CourseResultsScreen`) shows "C u r s o:" with a colon and no "(Ej."
/// hint instead (see `course_results::course_header_pattern`).
pub(super) const CURSO_DETECT_HINT: &str = "C u r s o  (Ej.";

/// A course with multiple sections prompts to narrow down to one before
/// landing on `CourseResultsScreen` -- same shape as `HorarioCursoScreen`
/// (free-text search, PF4 to exit), just for a section number instead of
/// a course code.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct HorarioSeccionScreen;

/// "(Ej." is unique to this search prompt, same reasoning as
/// `CURSO_DETECT_HINT`.
pub(super) const SECCION_DETECT_HINT: &str = "S e c c i o n  (Ej.";

#[cfg(test)]
mod tests {
    use super::*;
    use crate::screens::{classify, TuiScreen};

    const HORARIO_SEMESTRE: &str =
        include_str!("../../../screens/menu_despliegue/horario_semestre.txt");
    const HORARIO_CURSO: &str = include_str!("../../../screens/menu_despliegue/horario_curso.txt");
    const HORARIO_PROCESANDO: &str =
        include_str!("../../../screens/menu_despliegue/horario_procesando.txt");
    const HORARIO_SECCION: &str =
        include_str!("../../../screens/menu_despliegue/horario_seccion.txt");
    const HORARIO_SECCION_PROCESANDO: &str =
        include_str!("../../../screens/menu_despliegue/horario_seccion_procesando.txt");

    #[test]
    fn classifies_horario_semester() {
        let TuiScreen::HorarioSemester(HorarioSemesterScreen { options }) =
            classify(HORARIO_SEMESTRE)
        else {
            panic!("expected HorarioSemester");
        };
        assert_eq!(
            options,
            vec![
                MenuOption { key: "1".into(), label: "1erVer".into() },
                MenuOption { key: "2".into(), label: "1erSem".into() },
                MenuOption { key: "3".into(), label: "2doSem".into() },
                MenuOption { key: "4".into(), label: "2doVer".into() },
            ]
        );
    }

    #[test]
    fn classifies_horario_curso() {
        assert!(matches!(classify(HORARIO_CURSO), TuiScreen::HorarioCurso(_)));
    }

    #[test]
    fn horario_procesando_classifies_as_processing() {
        // The "Programa en Proceso" marquee overlays the course-search
        // prompt -- `classify` surfaces it as `Processing` rather than the
        // underlying `HorarioCurso`, since the remote is still computing
        // and will redraw again on its own (see `TuiScreen::Processing`).
        assert_eq!(classify(HORARIO_PROCESANDO), TuiScreen::Processing);
    }

    #[test]
    fn classifies_horario_seccion() {
        assert!(matches!(classify(HORARIO_SECCION), TuiScreen::HorarioSeccion(_)));
    }

    #[test]
    fn horario_seccion_procesando_classifies_as_processing() {
        // Same "Programa en Proceso" marquee as
        // `horario_procesando_classifies_as_processing`, this time shown
        // while narrowing a multi-section course down to one.
        assert_eq!(classify(HORARIO_SECCION_PROCESANDO), TuiScreen::Processing);
    }
}
