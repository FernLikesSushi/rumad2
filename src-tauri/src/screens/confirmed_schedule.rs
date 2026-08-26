//! The student's confirmed enrollment (`Matricula`'s own `[CONFIRMADA]`
//! report, shown after actually confirming a schedule): one row per
//! enrolled course with its section, credits, room, and meeting days/
//! times. Distinct from `CourseResultsScreen` (every section of *one*
//! course code, a lookup) and `MatriculaScreen` (the interactive course
//! list, which carries no room/schedule/professor data at all).

use async_trait::async_trait;
use regex::Regex;
use serde::Serialize;
use std::sync::OnceLock;

use super::scrape::{split_schedule, Meeting};
use super::RumadScreen;
use crate::ssh::session::TuiSession;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfirmedCourse {
    pub course: String,
    pub section: String,
    /// 0 for the one real transcript row missing this value entirely
    /// (see `parse_row`'s doc comment) rather than a genuine "0 credits"
    /// course.
    pub credits: u8,
    /// Can itself contain a space (e.g. "S 104A"), same as
    /// `CourseSection::room`.
    pub room: String,
    pub professor: String,
    pub meetings: Vec<Meeting>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ConfirmedScheduleScreen {
    pub courses: Vec<ConfirmedCourse>,
}

#[async_trait]
impl RumadScreen for ConfirmedScheduleScreen {
    /// Read-only report -- its own footer is just "Oprima <<Enter>> para
    /// Finalizar".
    async fn select(&self, _session: &mut TuiSession, _key: &str) -> anyhow::Result<()> {
        anyhow::bail!("ConfirmedSchedule has no selectable options; use continue_screen() instead")
    }

    /// No confirmed exit keystroke for this screen either (same situation
    /// as `WeeklyScheduleScreen`) -- left unimplemented rather than
    /// guessing.
    async fn exit(&self, _session: &mut TuiSession) -> anyhow::Result<()> {
        anyhow::bail!(
            "ConfirmedSchedule's exit keystroke isn't confirmed live; try continue_screen() instead"
        )
    }

    fn can_exit(&self) -> bool {
        false
    }

    fn can_continue(&self) -> bool {
        true
    }
}

pub(super) const DETECT_HINT: &str = "[CONFIRMADA]";

fn row_pattern() -> &'static Regex {
    static PATTERN: OnceLock<Regex> = OnceLock::new();
    // Course code (two words, e.g. "BIOL 3045") then section (one word).
    PATTERN.get_or_init(|| Regex::new(r"^\s*(\S+\s+\S+)\s+(\S+)\s+(.*)$").unwrap())
}

/// Parses one course row, e.g. " EDFI 3645  041    2    CM210  LW      10:00-10:50   FIGUEROA ROBLES, IRIS A."
///
/// Columns look nominally fixed-width from the header/separator, but
/// aren't really: a blank credits value (confirmed live -- "BIOL 3045"'s
/// own row has none) shifts the room text left into where credits would
/// otherwise sit, so fixed char offsets (like `course_results.rs` uses)
/// would misparse that row. Parsed by structure instead: after course
/// code + section, the day-letters+time-range span (`scrape::
/// split_schedule`) is the one unambiguous anchor in the rest of the
/// line -- whatever's between section and that span is credits+room (a
/// leading single digit is credits, if present at all), and whatever's
/// after is the professor (blank for "BIOL 3045", also confirmed live).
/// A line with no such span (header, separator, blank, the "* Cursos"
/// summary, the legend) has nothing to anchor on and is skipped.
fn parse_row(line: &str) -> Option<ConfirmedCourse> {
    let captures = row_pattern().captures(line)?;
    let course = captures[1].to_string();
    let section = captures[2].to_string();
    let rest = &captures[3];

    let (before, meetings, professor) = split_schedule(rest)?;

    let (credits, room) = match before.chars().next() {
        Some(c) if c.is_ascii_digit() => {
            let digit_end = before
                .find(|c: char| !c.is_ascii_digit())
                .unwrap_or(before.len());
            (before[..digit_end].parse().unwrap_or(0), before[digit_end..].trim().to_string())
        }
        _ => (0, before.to_string()),
    };

    Some(ConfirmedCourse {
        course,
        section,
        credits,
        room,
        professor: professor.to_string(),
        meetings,
    })
}

pub(super) fn scrape(raw: &str) -> ConfirmedScheduleScreen {
    ConfirmedScheduleScreen {
        courses: raw.lines().filter_map(parse_row).collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::screens::{classify, TuiScreen};

    const HORARIO_CONFIRMADO: &str =
        include_str!("../../../screens/matricula/horario_confirmado.txt");

    #[test]
    fn classifies_confirmed_schedule() {
        let TuiScreen::ConfirmedSchedule(ConfirmedScheduleScreen { courses }) =
            classify(HORARIO_CONFIRMADO).screen
        else {
            panic!("expected ConfirmedSchedule");
        };
        assert_eq!(
            courses,
            vec![
                ConfirmedCourse {
                    course: "BIOL 3045".to_string(),
                    section: "1".to_string(),
                    credits: 0,
                    room: "S 104A".to_string(),
                    professor: String::new(),
                    meetings: vec![Meeting {
                        day: 3,
                        start_minutes: 12 * 60 + 30,
                        end_minutes: 14 * 60 + 20
                    }],
                },
                ConfirmedCourse {
                    course: "EDFI 3645".to_string(),
                    section: "041".to_string(),
                    credits: 2,
                    room: "CM210".to_string(),
                    professor: "FIGUEROA ROBLES, IRIS A.".to_string(),
                    meetings: vec![
                        Meeting { day: 1, start_minutes: 10 * 60, end_minutes: 10 * 60 + 50 },
                        Meeting { day: 3, start_minutes: 10 * 60, end_minutes: 10 * 60 + 50 },
                    ],
                },
                ConfirmedCourse {
                    course: "FILO 4045".to_string(),
                    section: "020".to_string(),
                    credits: 3,
                    room: "CH119".to_string(),
                    professor: "THE LORAX".to_string(),
                    meetings: vec![
                        Meeting { day: 1, start_minutes: 8 * 60 + 30, end_minutes: 9 * 60 + 20 },
                        Meeting { day: 3, start_minutes: 8 * 60 + 30, end_minutes: 9 * 60 + 20 },
                        Meeting { day: 5, start_minutes: 8 * 60 + 30, end_minutes: 9 * 60 + 20 },
                    ],
                },
                ConfirmedCourse {
                    course: "INGE 3045".to_string(),
                    section: "086".to_string(),
                    credits: 3,
                    room: "S 305".to_string(),
                    professor: "CAVE JOHNSON".to_string(),
                    meetings: vec![
                        Meeting { day: 2, start_minutes: 14 * 60, end_minutes: 15 * 60 + 15 },
                        Meeting { day: 4, start_minutes: 14 * 60, end_minutes: 15 * 60 + 15 },
                    ],
                },
            ]
        );
    }
}
