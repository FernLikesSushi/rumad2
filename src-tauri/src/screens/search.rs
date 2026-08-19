//! `SearchScreen` -- the shape shared by `MenuDespliegue` option 6's two
//! free-text search steps ("Horario de cursos disponibles en Matricula"):
//! search a course code, then (if it has multiple sections) search a
//! section number to narrow down to one. Both are structurally and
//! behaviorally identical (no data of their own, no selectable options,
//! PF4 to exit) -- only `SearchKind` differs, which the frontend uses to
//! pick its own hint text and title.

use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SearchKind {
    /// Free-text course-code search, e.g. "C u r s o  (Ej. QUIM3001L)
    /// Puede indicar solo MATERIA             [PF4=(9)Fin]". Submitting a
    /// course with exactly one section lands on `CourseResultsScreen`
    /// directly; one with several lands on `HorarioSeccion` instead, to
    /// narrow down to one first.
    HorarioCurso,
    /// Free-text section-number search, shown only for a course with
    /// multiple sections, e.g. "S e c c i o n  (Ej. 001#)
    /// [PF4=(9)Fin]". Submitting lands on `CourseResultsScreen`.
    HorarioSeccion,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SearchScreen {
    pub search: SearchKind,
}

/// "C u r s o  (Ej." is unique to that search prompt -- the results
/// screen (`CourseResultsScreen`) shows "C u r s o:" with a colon and no
/// "(Ej." hint instead (see `course_results::course_header_pattern`).
pub(super) const HORARIO_CURSO_DETECT_HINT: &str = "C u r s o  (Ej.";

/// "S e c c i o n  (Ej." is unique to that search prompt, same reasoning
/// as `HORARIO_CURSO_DETECT_HINT`.
pub(super) const HORARIO_SECCION_DETECT_HINT: &str = "S e c c i o n  (Ej.";

#[cfg(test)]
mod tests {
    use crate::screens::{classify, SearchKind, TuiScreen};

    const HORARIO_CURSO: &str = include_str!("../../../screens/menu_despliegue/horario_curso.txt");
    const HORARIO_PROCESANDO: &str =
        include_str!("../../../screens/menu_despliegue/horario_procesando.txt");
    const HORARIO_SECCION: &str =
        include_str!("../../../screens/menu_despliegue/horario_seccion.txt");
    const HORARIO_SECCION_PROCESANDO: &str =
        include_str!("../../../screens/menu_despliegue/horario_seccion_procesando.txt");

    #[test]
    fn classifies_horario_curso() {
        let TuiScreen::Search(screen) = classify(HORARIO_CURSO).screen else {
            panic!("expected Search");
        };
        assert_eq!(screen.search, SearchKind::HorarioCurso);
    }

    #[test]
    fn horario_procesando_is_still_the_same_search_prompt() {
        // The transient "Programa en Proceso" status (see `Dialog::Processing`)
        // doesn't change which screen this is; the course-search prompt is
        // still live underneath it.
        let TuiScreen::Search(screen) = classify(HORARIO_PROCESANDO).screen else {
            panic!("expected Search");
        };
        assert_eq!(screen.search, SearchKind::HorarioCurso);
    }

    #[test]
    fn classifies_horario_seccion() {
        let TuiScreen::Search(screen) = classify(HORARIO_SECCION).screen else {
            panic!("expected Search");
        };
        assert_eq!(screen.search, SearchKind::HorarioSeccion);
    }

    #[test]
    fn horario_seccion_procesando_is_still_the_same_search_prompt() {
        let TuiScreen::Search(screen) = classify(HORARIO_SECCION_PROCESANDO).screen else {
            panic!("expected Search");
        };
        assert_eq!(screen.search, SearchKind::HorarioSeccion);
    }
}
