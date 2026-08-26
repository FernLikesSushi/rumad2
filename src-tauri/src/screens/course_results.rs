//! Course/section search results (`MENU DESPLIEGUE` -> "Horario de cursos
//! disponibles en Matricula"): a read-only table of every open section for
//! one course code, with room/schedule/professor/capacity. Distinct from
//! `Matricula`'s own course list -- this is a lookup, not the student's
//! actual schedule, and has no action menu, just "Enter to continue" or
//! PF4 to leave.

use async_trait::async_trait;
use regex::Regex;
use serde::Serialize;
use std::sync::OnceLock;

use super::scrape::{parse_schedule, Meeting};
use super::RumadScreen;
use crate::ssh::key::Key;
use crate::ssh::session::TuiSession;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct CourseSection {
    /// May carry a trailing " *" (e.g. "101 *") marking a closed/full
    /// section -- passed through as-is rather than parsed into a flag,
    /// same treatment as `available`'s trailing "-".
    pub section: String,
    /// Room, or empty for sections scheduled "Por acuerdo" (by
    /// arrangement, no fixed room/time). Can itself contain a space (e.g.
    /// "S 121" -- building letter, room number) -- passed through as-is
    /// rather than split further.
    pub room: String,
    /// Raw schedule string as printed in the table, e.g. "LWV      9:15-10:05"
    pub schedule: String,
    pub credits: u8,
    pub professor: String,
    pub capacity: u8,
    pub used: u8,
    /// Seats left, or negative when oversubscribed by that many (e.g. the
    /// remote's own "35-" becomes -35).
    pub available: i16,
    /// `schedule` parsed into one entry per meeting day -- see
    /// `scrape::parse_schedule`. Empty for "Por acuerdo" (by arrangement)
    /// sections or anything else that doesn't match the expected shape.
    pub meetings: Vec<Meeting>,
}

/// Read-only search results for one course code, e.g. "C u r s o:  HIST
/// 3220        ---> INT HIST:ENF SOCIAL" plus a table of open sections.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CourseResultsScreen {
    pub course_code: String,
    pub course_title: String,
    pub sections: Vec<CourseSection>,
}

#[async_trait]
impl RumadScreen for CourseResultsScreen {
    /// No numbered options on this screen, just "Enter to continue" or
    /// PF4 to leave.
    async fn select(&self, _session: &mut TuiSession, _key: &str) -> anyhow::Result<()> {
        anyhow::bail!("CourseResults has no selectable options; use continue_screen() or exit()")
    }

    /// PF4 exits this screen specifically -- confirmed live from its own
    /// footer ("< Oprima Enter o [PF4(9)=Fin] >").
    async fn exit(&self, session: &mut TuiSession) -> anyhow::Result<()> {
        session.send_key(Key::F4).await
    }

    fn can_continue(&self) -> bool {
        true
    }
}

fn course_header_pattern() -> &'static Regex {
    static PATTERN: OnceLock<Regex> = OnceLock::new();
    // "C u r s o:  HIST 3455        ---> INTRODUCCION A LA HISTORIA" --
    // spaced out letters like "M A T R I C U L A" elsewhere in this remote.
    PATTERN.get_or_init(|| Regex::new(r"(?m)^C u r s o:\s*(\S+ \S+)\s*--->\s*(.+?)\s*$").unwrap())
}

pub(super) fn detect_course(raw: &str) -> Option<(String, String)> {
    let captures = course_header_pattern().captures(raw)?;
    Some((captures[1].to_string(), captures[2].trim().to_string()))
}

// Section rows are fixed-width VMS report columns, not whitespace-
// delimited -- confirmed against a real transcript where the "Salon"
// column itself contains an internal space ("S 121"), which broke an
// earlier whitespace-splitting approach. Boundaries below are the byte --
// well, char -- offsets where real data consistently lines up regardless
// of content length (room name, professor name, ...):
// Sec.[0..6) Salon[6..13) Periodos[13..35) Crd.[35..39) Profesor[39..64)
// Cap.[64..68) Uti.[68..74) Disp.[74..).
const SECTION_COL: (usize, usize) = (0, 6);
const ROOM_COL: (usize, usize) = (6, 13);
const SCHEDULE_COL: (usize, usize) = (13, 35);
const CREDITS_COL: (usize, usize) = (35, 39);
const PROFESSOR_COL: (usize, usize) = (39, 64);
const CAPACITY_COL: (usize, usize) = (64, 68);
const USED_COL: (usize, usize) = (68, 74);

fn column(chars: &[char], (start, end): (usize, usize)) -> String {
    if start >= chars.len() {
        return String::new();
    }
    chars[start..end.min(chars.len())]
        .iter()
        .collect::<String>()
        .trim()
        .to_string()
}

fn is_digits(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_ascii_digit())
}

/// Parses one line as a section row, rejecting anything that isn't one
/// (header, separator, "* Totales" summary, blank lines) by requiring
/// digits to actually land in the credits/capacity/used/available
/// columns -- none of those other line kinds have digits there.
fn parse_section_row(line: &str) -> Option<CourseSection> {
    let chars: Vec<char> = line.chars().collect();
    let section = column(&chars, SECTION_COL);
    let credits = column(&chars, CREDITS_COL);
    let capacity = column(&chars, CAPACITY_COL);
    let used = column(&chars, USED_COL);
    let available = column(&chars, (74, chars.len()));
    let available_digits = available.strip_suffix('-').unwrap_or(&available);

    if section.is_empty() || !is_digits(&credits) || !is_digits(&capacity) || !is_digits(&used) {
        return None;
    }
    if !is_digits(available_digits) {
        return None;
    }

    let schedule = column(&chars, SCHEDULE_COL);
    let meetings = parse_schedule(&schedule);
    let available: i16 = match available.strip_suffix('-') {
        Some(digits) => -digits.parse::<i16>().unwrap_or(0),
        None => available.parse().unwrap_or(0),
    };

    Some(CourseSection {
        section,
        room: column(&chars, ROOM_COL),
        schedule,
        credits: credits.parse().unwrap_or(0),
        professor: column(&chars, PROFESSOR_COL),
        capacity: capacity.parse().unwrap_or(0),
        used: used.parse().unwrap_or(0),
        available,
        meetings,
    })
}

pub(super) fn scrape_sections(raw: &str) -> Vec<CourseSection> {
    raw.lines().filter_map(parse_section_row).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::screens::{classify, TuiScreen};

    const SIMPLE: &str =
        include_str!("../../../screens/menu_despliegue/horario_resultados_simple.txt");
    const MULTI: &str =
        include_str!("../../../screens/menu_despliegue/horario_resultados_multi.txt");
    const CLOSED_SECTION: &str =
        include_str!("../../../screens/menu_despliegue/horario_resultados_seccion_cerrada.txt");
    const LABORATORIO: &str =
        include_str!("../../../screens/menu_despliegue/horario_resultados_laboratorio.txt");

    #[test]
    fn classifies_single_section_results() {
        let TuiScreen::CourseResults(CourseResultsScreen {
            course_code,
            course_title,
            sections,
        }) = classify(SIMPLE).screen
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
                credits: 3,
                professor: "NOEMI TORRES VEGA".to_string(),
                capacity: 32,
                used: 32,
                available: 0,
                meetings: vec![
                    Meeting { day: 2, start_minutes: 8 * 60, end_minutes: 9 * 60 + 15 },
                    Meeting { day: 4, start_minutes: 8 * 60, end_minutes: 9 * 60 + 15 },
                ],
            }]
        );
    }

    #[test]
    fn classifies_multi_section_results_including_por_acuerdo() {
        let TuiScreen::CourseResults(CourseResultsScreen {
            course_code,
            course_title,
            sections,
        }) = classify(MULTI).screen
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
        assert_eq!(por_acuerdo.available, -35);
        assert_eq!(por_acuerdo.meetings, vec![]);

        let scheduled = &sections[1];
        assert_eq!(scheduled.section, "025");
        assert_eq!(scheduled.room, "CH318A");
        assert_eq!(scheduled.schedule, "LWV      9:15-10:05");
        assert_eq!(scheduled.professor, "RAMON DIAZ MELENDEZ");
        assert_eq!(
            scheduled.meetings,
            vec![
                Meeting {
                    day: 1,
                    start_minutes: 9 * 60 + 15,
                    end_minutes: 10 * 60 + 5
                },
                Meeting {
                    day: 3,
                    start_minutes: 9 * 60 + 15,
                    end_minutes: 10 * 60 + 5
                },
                Meeting {
                    day: 5,
                    start_minutes: 9 * 60 + 15,
                    end_minutes: 10 * 60 + 5
                },
            ]
        );

        // "055" ("LWV     12:15- 1:05pm") crosses noon with an explicit
        // marker -- both start (forced noon via hour==12) and end become
        // PM.
        let crosses_noon_marked = &sections[3];
        assert_eq!(crosses_noon_marked.section, "055");
        assert_eq!(
            crosses_noon_marked.meetings,
            vec![
                Meeting {
                    day: 1,
                    start_minutes: 12 * 60 + 15,
                    end_minutes: 13 * 60 + 5
                },
                Meeting {
                    day: 3,
                    start_minutes: 12 * 60 + 15,
                    end_minutes: 13 * 60 + 5
                },
                Meeting {
                    day: 5,
                    start_minutes: 12 * 60 + 15,
                    end_minutes: 13 * 60 + 5
                },
            ]
        );
    }

    #[test]
    fn classifies_closed_section_with_no_assigned_professor() {
        let TuiScreen::CourseResults(CourseResultsScreen {
            course_code,
            course_title,
            sections,
        }) = classify(CLOSED_SECTION).screen
        else {
            panic!("expected CourseResults");
        };
        assert_eq!(course_code, "ELEC 3101");
        assert_eq!(course_title, "CIRCUITOS ELECTRICOS I");
        assert_eq!(
            sections,
            vec![CourseSection {
                section: "101 *".to_string(),
                room: "EE204".to_string(),
                schedule: "LW       4:00- 5:15pm".to_string(),
                credits: 3,
                professor: String::new(),
                capacity: 45,
                used: 45,
                available: 0,
                meetings: vec![
                    Meeting {
                        day: 1,
                        start_minutes: 16 * 60,
                        end_minutes: 17 * 60 + 15
                    },
                    Meeting {
                        day: 3,
                        start_minutes: 16 * 60,
                        end_minutes: 17 * 60 + 15
                    },
                ],
            }]
        );
    }

    #[test]
    fn classifies_lab_sections_with_multi_word_rooms() {
        let TuiScreen::CourseResults(CourseResultsScreen {
            course_code,
            course_title,
            sections,
        }) = classify(LABORATORIO).screen
        else {
            panic!("expected CourseResults");
        };
        assert_eq!(course_code, "BIOL 3011L");
        assert_eq!(course_title, "LABORATORIO DE BIOLOGIA I");
        assert_eq!(sections.len(), 6);

        let first = &sections[0];
        assert_eq!(first.section, "015L*");
        assert_eq!(first.room, "L 210");
        assert_eq!(first.schedule, "W        8:15- 9:05");
        assert_eq!(first.credits, 0);
        assert_eq!(first.professor, "");
        assert_eq!(first.capacity, 28);
        assert_eq!(first.used, 17);
        assert_eq!(first.available, 11);
        assert_eq!(
            first.meetings,
            vec![Meeting { day: 3, start_minutes: 8 * 60 + 15, end_minutes: 9 * 60 + 5 }]
        );

        let second = &sections[1];
        assert_eq!(second.section, "016L*");
        assert_eq!(second.room, "L 208A");

        // "055L*" ("W       10:15- 1:05pm") crosses noon without a marker
        // on the start -- 10:15am, not 10:15pm.
        let crosses_noon = &sections[4];
        assert_eq!(crosses_noon.section, "055L*");
        assert_eq!(
            crosses_noon.meetings,
            vec![Meeting { day: 3, start_minutes: 10 * 60 + 15, end_minutes: 13 * 60 + 5 }]
        );

        let oversubscribed = &sections[5];
        assert_eq!(oversubscribed.section, "056L*");
        assert_eq!(oversubscribed.available, -13);
    }
}
