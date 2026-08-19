//! Course/section search results (`MENU DESPLIEGUE` -> "Horario de cursos
//! disponibles en Matricula"): a read-only table of every open section for
//! one course code, with room/schedule/professor/capacity. Distinct from
//! `Matricula`'s own course list -- this is a lookup, not the student's
//! actual schedule, and has no action menu, just "Enter to continue" or
//! PF4 to leave.

use regex::Regex;
use serde::Serialize;
use std::sync::OnceLock;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct CourseSection {
    pub section: String,
    /// Room, or empty for sections scheduled "Por acuerdo" (by
    /// arrangement, no fixed room/time).
    pub room: String,
    pub schedule: String,
    pub credits: String,
    pub professor: String,
    pub capacity: String,
    pub used: String,
    /// Seats left, or a count with a trailing "-" meaning oversubscribed
    /// by that many (e.g. "35-") -- passed through as-is rather than
    /// parsed, since the sign carries meaning.
    pub available: String,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CourseResultsScreen {
    pub course_code: String,
    pub course_title: String,
    pub sections: Vec<CourseSection>,
}

fn course_header_pattern() -> &'static Regex {
    static PATTERN: OnceLock<Regex> = OnceLock::new();
    // "C u r s o:  HIST 3455        ---> INTRODUCCION A LA HISTORIA" --
    // spaced out letters like "M A T R I C U L A" elsewhere in this remote.
    PATTERN.get_or_init(|| Regex::new(r"(?m)^C u r s o:\s*(\S+ \S+)\s*--->\s*(.+?)\s*$").unwrap())
}

fn section_row_pattern() -> &'static Regex {
    static PATTERN: OnceLock<Regex> = OnceLock::new();
    // Columns are whitespace-separated except "room"+"schedule" together,
    // which can itself contain an internal space (e.g. "12:15- 1:05pm"),
    // so room+schedule is captured as one non-greedy blob up to the next
    // isolated whitespace-bounded digit run (credits). Anchoring the tail
    // on three more digit groups (credits, capacity, used) plus a
    // digits-with-optional-trailing-dash group (available) is what keeps
    // this from matching the header/separator/totals rows, none of which
    // have that shape.
    PATTERN.get_or_init(|| {
        Regex::new(r"(?m)^(\S+)\s+(.+?)\s+(\d+)\s+([A-Z][A-Za-z .,'-]*?)\s+(\d+)\s+(\d+)\s+(\d+-?)\s*$")
            .unwrap()
    })
}

pub(super) fn detect_course(raw: &str) -> Option<(String, String)> {
    let captures = course_header_pattern().captures(raw)?;
    Some((captures[1].to_string(), captures[2].trim().to_string()))
}

pub(super) fn scrape_sections(raw: &str) -> Vec<CourseSection> {
    section_row_pattern()
        .captures_iter(raw)
        .map(|c| CourseSection {
            section: c[1].to_string(),
            room: String::new(),
            schedule: c[2].trim().to_string(),
            credits: c[3].to_string(),
            professor: c[4].trim().to_string(),
            capacity: c[5].to_string(),
            used: c[6].to_string(),
            available: c[7].to_string(),
        })
        .map(split_room_from_schedule)
        .collect()
}

/// The row regex can't cleanly separate "room" from "schedule" up front
/// (both live in one non-greedy capture, see `section_row_pattern`), so
/// this splits the room token (present) from the rest once the split
/// point is unambiguous: a section scheduled "Por acuerdo" has no room at
/// all, otherwise the first word is the room.
fn split_room_from_schedule(mut section: CourseSection) -> CourseSection {
    if section.schedule == "Por acuerdo" {
        return section;
    }
    if let Some((room, rest)) = section.schedule.split_once(char::is_whitespace) {
        section.room = room.to_string();
        section.schedule = rest.trim().to_string();
    }
    section
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::screens::{classify, TuiScreen};

    const SIMPLE: &str = include_str!("../../../screens/menu_despliegue/horario_resultados_simple.txt");
    const MULTI: &str = include_str!("../../../screens/menu_despliegue/horario_resultados_multi.txt");

    #[test]
    fn classifies_single_section_results() {
        let TuiScreen::CourseResults(CourseResultsScreen {
            course_code,
            course_title,
            sections,
        }) = classify(SIMPLE)
        else {
            panic!("expected CourseResults");
        };
        assert_eq!(course_code, "HIST 3220");
        assert_eq!(course_title, "INT HIST:ENF SOCIAL");
        assert_eq!(
            sections,
            vec![CourseSection {
                section: "032".to_string(),
                room: "CH316".to_string(),
                schedule: "MJ       8:00- 9:15".to_string(),
                credits: "3".to_string(),
                professor: "NOEMI TORRES VEGA".to_string(),
                capacity: "32".to_string(),
                used: "32".to_string(),
                available: "00".to_string(),
            }]
        );
    }

    #[test]
    fn classifies_multi_section_results_including_por_acuerdo() {
        let TuiScreen::CourseResults(CourseResultsScreen {
            course_code,
            course_title,
            sections,
        }) = classify(MULTI)
        else {
            panic!("expected CourseResults");
        };
        assert_eq!(course_code, "HIST 3455");
        assert_eq!(course_title, "INTRODUCCION A LA HISTORIA");
        assert_eq!(sections.len(), 8);

        let por_acuerdo = &sections[0];
        assert_eq!(por_acuerdo.section, "005D");
        assert_eq!(por_acuerdo.room, "");
        assert_eq!(por_acuerdo.schedule, "Por acuerdo");
        assert_eq!(por_acuerdo.professor, "ANGEL MORALES CRUZ");
        assert_eq!(por_acuerdo.available, "35-");

        let scheduled = &sections[1];
        assert_eq!(scheduled.section, "025");
        assert_eq!(scheduled.room, "CH318A");
        assert_eq!(scheduled.schedule, "LWV      9:15-10:05");
        assert_eq!(scheduled.professor, "RAMON DIAZ MELENDEZ");
    }
}
