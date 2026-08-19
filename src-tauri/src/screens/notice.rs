//! The remote's generalized "<< message >>" advisory pattern -- distinct
//! from the specific "no esta disponible" rejection, which `classify`
//! checks directly since it needs to win at high priority (see
//! `TuiScreen::or_err`).

use regex::Regex;
use std::sync::OnceLock;

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

/// Extracts a "<< message >>" advisory line, if the screen has one.
/// Deliberately excludes bare key-name hints like a lone "<<Enter>>" line
/// (real advisories are full phrases, not a single word) -- see
/// `bracketed_notice_pattern`.
pub(super) fn extract_bracketed_notice(raw: &str) -> Option<String> {
    raw.lines().find_map(|line| {
        let captures = bracketed_notice_pattern().captures(line.trim())?;
        let message = captures[1].trim();
        message.contains(' ').then(|| message.to_string())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::screens::{classify, NoticeScreen, TuiScreen, UnknownScreen};

    const EVALUO_PAGO: &str = include_str!("../../../screens/matricula/evaluo_pago.txt");
    const HORARIOS_DE_SECCION: &str =
        include_str!("../../../screens/matricula/horarios_de_seccion.txt");

    #[test]
    fn bracketed_notice_on_an_unmodeled_screen_is_extracted_cleanly() {
        // evaluo_pago.txt (the payment/invoice screen) isn't specifically
        // modeled, but its "<< ... >>" disclaimer should still be pulled
        // out as a Notice message rather than left buried in raw text.
        let TuiScreen::Notice(NoticeScreen { message, .. }) = classify(EVALUO_PAGO) else {
            panic!("expected Notice");
        };
        assert_eq!(
            message,
            "Esta factura NO ES OFICIAL, cargos y pagos sujetos a verificacion"
        );
        // Not a "no esta disponible" rejection, so it must not become a
        // command error.
        assert!(TuiScreen::Notice(NoticeScreen {
            message,
            raw: String::new()
        })
        .or_err()
        .is_ok());
    }

    #[test]
    fn triple_angle_bracket_prompt_is_not_mistaken_for_an_advisory() {
        // horarios_de_seccion.txt ends with "<<<  Oprima Return  >>>" --
        // triple angle brackets, a plain "press enter" prompt, not a "<<
        // message >>" advisory. It isn't a modeled screen either, so this
        // should land in Unknown with the raw text intact, not a mangled
        // Notice with leftover bracket characters.
        assert!(matches!(
            classify(HORARIOS_DE_SECCION),
            TuiScreen::Unknown(UnknownScreen { .. })
        ));
    }

    #[test]
    fn extract_bracketed_notice_rejects_bare_key_hints() {
        assert_eq!(extract_bracketed_notice("<<Enter>>"), None);
        assert_eq!(
            extract_bracketed_notice("Oprima <<Enter>> para Continuar"),
            None
        );
        assert_eq!(extract_bracketed_notice("<<<  Oprima Return  >>>"), None);
    }
}
