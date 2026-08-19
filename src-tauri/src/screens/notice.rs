//! The remote's generalized advisory patterns -- lines that are *entirely*
//! wrapped in a marker on both sides, e.g. "<< message >>" or "*** message
//! ***". `classify` runs `detect_dialog` (which calls these) and
//! `classify_screen` independently against the same raw text and combines
//! the results (see `ClassifiedScreen`), so unlike the old design, these
//! don't need to worry about "swallowing" a specific screen type -- only
//! about not misreading a screen's own boilerplate as an advisory.
//!
//! - `extract_starred_notice` ("*** message ***") is, in every
//!   live-observed instance but one, actually a rejection ("Curso
//!   incorrecto", "NO tiene Matricula", "Curso NO Existe en Archivo
//!   MTR-HORARIO") -- shown *on top of* the same prompt that triggered it
//!   (`SearchScreen`'s own footer stays on screen underneath). The one
//!   exception, `CourseResultsScreen`'s own "*** Horarios de Matricula
//!   ***" header, is a plain title, not a rejection -- excluded by exact
//!   text match rather than by ordering tricks, now that dialog detection
//!   doesn't depend on which screen type also matched.
//! - `extract_bracketed_or_boxed_notice` ("<< message >>", or a boxed
//!   "****...****" variant) is genuinely just informational in every
//!   observed instance (a disclaimer, a redirect) rather than a rejection.

use regex::Regex;
use std::sync::OnceLock;

/// `CourseResultsScreen`'s own header, not a rejection -- see this
/// module's doc comment.
const COURSE_RESULTS_TITLE: &str = "Horarios de Matricula";

fn bracketed_notice_pattern() -> &'static Regex {
    static PATTERN: OnceLock<Regex> = OnceLock::new();
    // A line that is *entirely* a "<< message >>" advisory, e.g. "<< NO
    // oprimir tecla <Enter> al entrar los datos >>" (note the harmless
    // nested single-bracket "<Enter>") or "<< Esta factura NO ES
    // OFICIAL... >>". Anchored to the whole (trimmed) line so it doesn't
    // match a bracketed key hint embedded mid-instruction, like "Oprima
    // <<Enter>> para Continuar". The `[^<>]` boundaries on the captured
    // content require the brackets to be *exactly* double -- distinct from
    // the remote's other, unrelated "<<<  Oprima Return  >>>" triple-angle
    // style (a plain "press enter" prompt, not an advisory), which would
    // otherwise partially match and capture a mangled leftover bracket.
    PATTERN.get_or_init(|| Regex::new(r"^<<([^<>].*?[^<>])>>$").unwrap())
}

fn starred_notice_pattern() -> &'static Regex {
    static PATTERN: OnceLock<Regex> = OnceLock::new();
    // A line that is *entirely* a "*** message ***" rejection, e.g. "***
    // Curso NO Existe en Archivo MTR-HORARIO ***". The `[^*]` boundaries
    // require exactly three stars on each side -- MENU PRINCIPAL's own
    // "***>>>  LEE tu Correo Electronico..." option line isn't anchored
    // (it starts with "1.", not "***") and doesn't end in "***" either, so
    // it can't collide with this pattern.
    PATTERN.get_or_init(|| Regex::new(r"^\*\*\*([^*].*?[^*])\*\*\*$").unwrap())
}

fn extract_matching_line(raw: &str, pattern: &Regex) -> Option<String> {
    raw.lines().find_map(|line| {
        let captures = pattern.captures(line.trim())?;
        let message = captures[1].trim();
        message.contains(' ').then(|| message.to_string())
    })
}

/// See this module's doc comment -- checked at high priority.
pub(super) fn extract_starred_notice(raw: &str) -> Option<String> {
    extract_matching_line(raw, starred_notice_pattern())
        .filter(|message| message != COURSE_RESULTS_TITLE)
}

/// See this module's doc comment -- checked at low priority. Deliberately
/// excludes bare key-name hints like a lone "<<Enter>>" line (real
/// advisories are full phrases, not a single word).
pub(super) fn extract_bracketed_or_boxed_notice(raw: &str) -> Option<String> {
    extract_matching_line(raw, bracketed_notice_pattern()).or_else(|| extract_boxed_notice(raw))
}

/// Extracts a boxed advisory -- a "****...****" border line, then one or
/// more "*  text  *" content lines, then a matching border again, e.g.
/// `screens/menu_despliegue/turno_seleccion.txt`'s "Su TURNO para
/// seleccion de cursos y secciones..." redirect box. Content lines are
/// joined into one message (each line's own internal run of spaces
/// collapsed to one, same as the box's visual word-wrapping) rather than
/// kept as separate lines, since this is one continuous advisory that just
/// happens to be wrapped across several rows to fit the box.
fn extract_boxed_notice(raw: &str) -> Option<String> {
    let has_border = raw
        .lines()
        .any(|line| !line.trim().is_empty() && line.trim().chars().all(|c| c == '*'));
    if !has_border {
        return None;
    }

    let parts: Vec<String> = raw
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.chars().all(|c| c == '*') {
                return None; // blank row, or the border itself
            }
            let inner = trimmed.strip_prefix('*')?.strip_suffix('*')?.trim();
            if inner.is_empty() || inner.chars().all(|c| c == '-') {
                return None; // blank interior row, or a decorative underline
            }
            Some(inner.split_whitespace().collect::<Vec<_>>().join(" "))
        })
        .collect();

    (!parts.is_empty()).then(|| parts.join(" "))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::screens::{classify, Dialog, TuiScreen, UnknownScreen};

    const EVALUO_PAGO: &str = include_str!("../../../screens/matricula/evaluo_pago.txt");
    const HORARIOS_DE_SECCION: &str =
        include_str!("../../../screens/matricula/horarios_de_seccion.txt");
    const TURNO_SELECCION: &str =
        include_str!("../../../screens/menu_despliegue/turno_seleccion.txt");

    #[test]
    fn bracketed_notice_on_an_unmodeled_screen_is_extracted_cleanly() {
        // evaluo_pago.txt (the payment/invoice screen) isn't specifically
        // modeled, but its "<< ... >>" disclaimer should still be pulled
        // out as a Dialog::Notice rather than left buried in raw text.
        let result = classify(EVALUO_PAGO);
        let Some(Dialog::Notice { message, .. }) = &result.dialog else {
            panic!("expected a Notice dialog");
        };
        assert_eq!(
            message,
            "Esta factura NO ES OFICIAL, cargos y pagos sujetos a verificacion"
        );
        // Not a "no esta disponible" rejection, so it must not become a
        // command error.
        assert!(result.or_err().is_ok());
    }

    #[test]
    fn triple_angle_bracket_prompt_is_not_mistaken_for_an_advisory() {
        // horarios_de_seccion.txt ends with "<<<  Oprima Return  >>>" --
        // triple angle brackets, a plain "press enter" prompt, not a "<<
        // message >>" advisory. It isn't a modeled screen either, so this
        // should land in Unknown with the raw text intact, not a mangled
        // Notice with leftover bracket characters.
        let result = classify(HORARIOS_DE_SECCION);
        assert!(matches!(
            result.screen,
            TuiScreen::Unknown(UnknownScreen { .. })
        ));
        assert_eq!(result.dialog, None);
    }

    #[test]
    fn extract_bracketed_notice_rejects_bare_key_hints() {
        assert_eq!(extract_bracketed_or_boxed_notice("<<Enter>>"), None);
        assert_eq!(
            extract_bracketed_or_boxed_notice("Oprima <<Enter>> para Continuar"),
            None
        );
        assert_eq!(
            extract_bracketed_or_boxed_notice("<<<  Oprima Return  >>>"),
            None
        );
    }

    #[test]
    fn starred_notice_is_extracted_cleanly() {
        // Live-observed: browsing a period the student has no enrolled
        // schedule for shows this in place of the usual course table.
        let raw = "*** NO tiene Matricula ***";
        let result = classify(raw);
        assert_eq!(
            result.dialog,
            Some(Dialog::Notice {
                message: "NO tiene Matricula".to_string(),
                raw: raw.to_string()
            })
        );
        // Informational, not a rejection -- must not become a command error.
        assert!(result.or_err().is_ok());
    }

    #[test]
    fn starred_notice_does_not_collide_with_main_menu_option_marker() {
        // MENU PRINCIPAL's own "***>>>  LEE tu Correo..." option line
        // isn't a "*** message ***" advisory (no closing "***", and it
        // isn't the whole trimmed line) -- must not be mistaken for one.
        assert_eq!(
            extract_starred_notice("1.  ***>>>  LEE tu Correo Electronico en ->  outlook.com"),
            None
        );
    }

    #[test]
    fn starred_notice_excludes_course_results_own_title() {
        // The one known non-rejection instance of the "*** ... ***"
        // marker -- see this module's doc comment.
        assert_eq!(
            extract_starred_notice("*** Horarios de Matricula ***"),
            None
        );
    }

    #[test]
    fn starred_rejection_wins_over_the_prompt_still_showing_underneath_it() {
        // Live-observed: searching a course/section that doesn't exist
        // redisplays the *same* HorarioSeccion prompt with the rejection
        // still on screen (grounded in a real capture), rather than
        // replacing it entirely -- both the (still-classified) screen and
        // the rejection dialog must come through together.
        let raw = "\
S e c c i o n  (Ej. 001#)                                          [PF4=(9)Fin]

               ***  Curso NO Existe en Archivo MTR-HORARIO  ***";
        let result = classify(raw);
        assert!(matches!(result.screen, TuiScreen::Search(_)));
        let Some(Dialog::Notice { message, .. }) = &result.dialog else {
            panic!("expected a Notice dialog");
        };
        assert_eq!(message, "Curso NO Existe en Archivo MTR-HORARIO");
        assert!(result.or_err().is_ok());
    }

    #[test]
    fn boxed_notice_is_extracted_and_joined_into_one_message() {
        // MENU DESPLIEGUE option 5 (Turno de seleccion...) -- a redirect
        // notice wrapped in a full "****...****"-bordered box, its text
        // word-wrapped across several rows. Nothing else is on this
        // screen, so it classifies as `Unknown` alongside the dialog.
        let result = classify(TURNO_SELECCION);
        assert_eq!(
            result.dialog,
            Some(Dialog::Notice {
                message: "Su TURNO para seleccion de cursos y secciones o las fechas de los examenes finales los podran ver a traves de Mi Portal Colegial en home.uprm.edu".to_string(),
                raw: TURNO_SELECCION.to_string(),
            })
        );
        // Informational, not a rejection -- must not become a command error.
        assert!(result.or_err().is_ok());
    }
}
