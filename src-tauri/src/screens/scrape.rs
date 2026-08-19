//! Generic text-scraping helpers shared across screen types -- not tied to
//! any one `TuiScreen` variant.

use regex::Regex;
use std::sync::OnceLock;

use super::MenuOption;

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
