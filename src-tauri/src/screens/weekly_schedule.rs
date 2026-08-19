//! The student's schedule rendered as a weekly grid (`MENU DESPLIEGUE` ->
//! "Horario de matricula grafico"): one row per time period, one column
//! per weekday, drawn with a real VT100 box-drawing-by-pipes table. Purely
//! informational -- distinct from `Matricula`'s own course list, which is
//! the interactive one with an action menu.
//!
//! A cell isn't always a course: some are blocked-off periods like "Hora
//! Universal", so cell text is kept as-is (trimmed) rather than parsed
//! into a course/section pair -- interpreting it further would mean
//! guessing at values not confirmed live.

use serde::Serialize;

use super::RumadScreen;
use crate::ssh::session::TuiSession;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ScheduleRow {
    pub period: String,
    /// One entry per day in `WeeklyScheduleScreen::days`, same order,
    /// empty string for a blank cell.
    pub days: Vec<String>,
}

/// The student's own schedule as a weekly day/time grid, e.g. a "Lunes",
/// "Martes", ... column header row and one `ScheduleRow` per time period
/// underneath.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct WeeklyScheduleScreen {
    pub days: Vec<String>,
    pub rows: Vec<ScheduleRow>,
}

impl RumadScreen for WeeklyScheduleScreen {
    /// No numbered options on this read-only grid.
    fn select(&self, _session: &mut TuiSession, _key: &str) -> anyhow::Result<()> {
        anyhow::bail!("WeeklySchedule has no selectable options; use line(\"\") instead")
    }

    /// Unlike `CourseResults`, the real capture of this screen has no
    /// visible footer/prompt line to confirm an exit keystroke against
    /// (see this module's doc comment on that transcript's capture
    /// issues) -- left unimplemented rather than guessing. `line` (bare
    /// Enter, inherited default) is a safe bet since every other screen in
    /// this app accepts it to continue/redraw.
    fn exit(&self, _session: &mut TuiSession) -> anyhow::Result<()> {
        anyhow::bail!(
            "WeeklySchedule's exit keystroke isn't confirmed live; try line(\"\") instead"
        )
    }

    fn can_exit(&self) -> bool {
        false
    }
}

/// A real capture of this screen had its last row missing a trailing
/// column (cut short during capture, not a remote rendering quirk) --
/// cells are padded to `days.len()` rather than assumed complete, so a
/// malformed row degrades to blank trailing cells instead of panicking or
/// silently dropping the row.
fn table_cells(line: &str) -> Option<Vec<String>> {
    let line = line.trim();
    let inner = line.strip_prefix('|')?.strip_suffix('|')?;
    Some(
        inner
            .split('|')
            .map(|cell| cell.trim().to_string())
            .collect(),
    )
}

pub(super) fn detect(raw: &str) -> bool {
    raw.contains("Periodos") && raw.contains("Lunes")
}

pub(super) fn scrape(raw: &str) -> WeeklyScheduleScreen {
    let mut lines = raw.lines();
    let days = lines
        .by_ref()
        .find_map(table_cells)
        .map(|cells| cells.into_iter().skip(1).collect::<Vec<_>>())
        .unwrap_or_default();

    let rows = lines
        .filter_map(table_cells)
        // Excludes both the header-separator row (dash-filled, no digits)
        // and the single-cell "Cursos N - Creditos N" totals row (has
        // digits, but no internal "|" dividers -- just one cell instead of
        // one-per-column).
        .filter(|cells| {
            cells.len() > 1
                && cells
                    .first()
                    .is_some_and(|period| period.chars().any(|c| c.is_ascii_digit()))
        })
        .map(|mut cells| {
            let period = cells.remove(0);
            cells.resize(days.len(), String::new());
            ScheduleRow {
                period,
                days: cells,
            }
        })
        .collect();

    WeeklyScheduleScreen { days, rows }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::screens::{classify, TuiScreen};

    const HORARIO_ESTIMADO: &str = include_str!("../../../screens/matricula/horario_estimado.txt");

    #[test]
    fn classifies_weekly_schedule_grid() {
        let TuiScreen::WeeklySchedule(WeeklyScheduleScreen { days, rows }) =
            classify(HORARIO_ESTIMADO).screen
        else {
            panic!("expected WeeklySchedule");
        };
        assert_eq!(
            days,
            vec![
                "Lunes",
                "Martes",
                "Mi\u{fffd}coles",
                "Jueves",
                "Viernes",
                "Sabado"
            ]
        );
        assert_eq!(rows.len(), 7);
        assert_eq!(rows[0].period, "8:30- 9:20");
        assert_eq!(rows[0].days[0], "FILO4045  - 020");
        assert_eq!(rows[0].days[1], "");
        assert_eq!(rows[2].days[1], "Hora Universal");
    }

    #[test]
    fn last_row_missing_a_trailing_column_is_padded_not_dropped() {
        // The real transcript's final row is missing its Sabado cell
        // (see this module's doc comment) -- confirm it still comes
        // through with the right length instead of panicking or being
        // silently dropped.
        let WeeklyScheduleScreen { days, rows } = scrape(HORARIO_ESTIMADO);
        let last = rows.last().expect("at least one row");
        assert_eq!(last.period, "4:30- 5:45 pm");
        assert_eq!(last.days.len(), days.len());
        assert_eq!(last.days.last().unwrap(), "");
    }
}
