//! Generic text-scraping helpers shared across screen types -- not tied to
//! any one `TuiScreen` variant.

use regex::Regex;
use serde::Serialize;
use std::sync::OnceLock;

use super::MenuOption;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Meeting {
    /// ISO 8601 weekday number (Monday=1 .. Sunday=7).
    pub day: u8,
    /// Minutes since midnight, 24-hour -- see `parse_schedule`'s doc
    /// comment for the am/pm heuristic.
    /// 0 is midnight, 60 is 1:00am, 12*60=720 is noon, 13*60=780 is 1:00pm, etc.
    pub start_minutes: u32,
    pub end_minutes: u32,
}

fn schedule_pattern() -> &'static Regex {
    static PATTERN: OnceLock<Regex> = OnceLock::new();
    // Not anchored with `^` -- `course_results.rs` passes an already-
    // isolated column so a match there is at position 0 regardless, but
    // `confirmed_schedule.rs`'s rows aren't fixed-width (see its own doc
    // comment) and need to search for this span within a larger string.
    PATTERN.get_or_init(|| {
        Regex::new(r"(?i)([LMWJVS]+)\s+(\d{1,2}):(\d{2})\s*-\s*(\d{1,2}):(\d{2})\s*(am|pm)?").unwrap()
    })
}

/// L=Lunes M=Martes W=Miercoles J=Jueves V=Viernes S=Sabado -- "W" stands
/// in for Miercoles since "M" is already Martes. Cross-checked against
/// `screens/matricula/horario_confirmado.txt` and `horario_estimado.txt`:
/// FILO4045's "LWV" and EDFI3645's "LW" there land on exactly the
/// Lunes/Miercoles/Viernes columns of the real weekly grid; "S" for
/// Sabado isn't itself exercised by either transcript but follows the
/// same pattern. Returned as an ISO 8601 weekday number (Monday=1).
fn iso_weekday(code: char) -> Option<u8> {
    match code.to_ascii_uppercase() {
        'L' => Some(1),
        'M' => Some(2),
        'W' => Some(3),
        'J' => Some(4),
        'V' => Some(5),
        'S' => Some(6),
        _ => None,
    }
}

fn resolve_hour(hour: u32, force_pm: bool) -> u32 {
    if hour == 12 {
        12
    } else if force_pm {
        hour + 12
    } else {
        hour
    }
}

/// Parses `schedule` (e.g. "MJ       8:00- 9:15", "LWV     12:15- 1:05pm")
/// into one `Meeting` per day letter. "Por acuerdo" (by arrangement) and
/// anything else that doesn't match yields an empty list.
///
/// Only one am/pm marker ever appears, trailing the end time, and doesn't
/// always apply to the start time too -- e.g. "10:15- 1:05pm" (a real
/// `horario_resultados_laboratorio.txt` row) is 10:15am-1:05pm, not
/// 10:15pm. Resolved by comparing the raw start/end hour digits: if
/// start > end (and neither is 12), the range crosses noon, so the start
/// stays AM and only the end becomes PM regardless of the marker;
/// otherwise the marker (if any) applies to both. `hour == 12` always
/// means noon on its own since these schedules never run past midnight.
/// Verified against every schedule string in `course_results.rs`'s and
/// `confirmed_schedule.rs`'s real transcripts.
pub(super) fn parse_schedule(schedule: &str) -> Vec<Meeting> {
    let Some(captures) = schedule_pattern().captures(schedule) else {
        return Vec::new();
    };
    let days = &captures[1];
    let start_hour: u32 = captures[2].parse().unwrap_or(0);
    let start_min: u32 = captures[3].parse().unwrap_or(0);
    let end_hour: u32 = captures[4].parse().unwrap_or(0);
    let end_min: u32 = captures[5].parse().unwrap_or(0);
    let marker_pm = captures
        .get(6)
        .is_some_and(|m| m.as_str().eq_ignore_ascii_case("pm"));

    let crosses_noon = start_hour != 12 && end_hour != 12 && start_hour > end_hour;
    let start_minutes = resolve_hour(start_hour, !crosses_noon && marker_pm) * 60 + start_min;
    let end_minutes = resolve_hour(end_hour, marker_pm || crosses_noon) * 60 + end_min;

    days.chars()
        .filter_map(iso_weekday)
        .map(|day| Meeting {
            day,
            start_minutes,
            end_minutes,
        })
        .collect()
}

/// Locates the day-letters+time-range span within `text` (if any) and
/// splits it into the text before, the parsed `Meeting`s, and the text
/// after -- for screens where the schedule isn't already isolated into
/// its own fixed column (`confirmed_schedule.rs`).
pub(super) fn split_schedule(text: &str) -> Option<(&str, Vec<Meeting>, &str)> {
    let m = schedule_pattern().find(text)?;
    let meetings = parse_schedule(m.as_str());
    Some((text[..m.start()].trim(), meetings, text[m.end()..].trim()))
}

fn option_pattern() -> &'static Regex {
    static PATTERN: OnceLock<Regex> = OnceLock::new();
    // Matches menu lines like "   2.  Seleccion de Secciones  (Matricula)"
    // or "0.  SALIR DEL SISTEMA" -- the numbering style used throughout
    // the real SISTEMA ESTUDIANTIL COLEGIAL screens.
    PATTERN.get_or_init(|| Regex::new(r"(?m)^\s*(\d+)\.\s+(.+?)\s*$").unwrap())
}

pub(super) fn scrape_options(raw: &str) -> Vec<MenuOption> {
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
pub(super) fn scrape_equals_options(raw: &str) -> Vec<MenuOption> {
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
