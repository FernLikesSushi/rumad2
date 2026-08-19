//! The real per-student authentication form (ID number, permanent access
//! code, last 4 of SSN, birth date) shown after selecting "Seleccion de
//! Secciones" -- separate from and unrelated to the SSH login, which every
//! student shares (`estudiante@...`).

use serde::Serialize;

use super::RumadScreen;
use crate::ssh::key::Key;
use crate::ssh::session::TuiSession;

/// One field of the `Login` form, in on-screen order. `label`/`hint` are
/// hardcoded to the correctly-accented Spanish text rather than parsed
/// from the remote
/// screen, because the real transcript's accented characters (í, ó, ú)
/// arrive mangled -- almost certainly a Latin-1/CP437 byte stream getting
/// decoded as UTF-8 somewhere in the vt100/ssh2 pipeline. That's a
/// pre-existing encoding issue worth investigating separately; since this
/// form's labels are static boilerplate (not per-user data), hardcoding
/// clean text sidesteps it rather than trying to fix the pipeline blind.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct LoginField {
    pub key: String,
    pub label: String,
    pub hint: String,
}

/// The real per-student authentication form, e.g. "Número de
/// Identificación", "Código de Acceso Permanente", "Seguro Social
/// (últimos 4)", "Fecha de Nacimiento" -- each field auto-advances with no
/// Enter (see `login`'s own doc comment).
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct LoginScreen {
    pub fields: Vec<LoginField>,
}

impl LoginScreen {
    /// Fills in the four fixed-width, auto-advancing fields, in on-screen
    /// order (id number, access code, SSN last 4, birth date).
    ///
    /// Whitespace is stripped from each before sending: these fields have
    /// no way to correct a mistake afterward (the remote explicitly warns
    /// against pressing Enter here), so a stray space would be written as
    /// a real character, consume a slot meant for a digit, and shift
    /// every field typed after it -- confirmed live, a
    /// whitespace-containing field produced a garbled value on screen.
    /// This is specific to `Login`'s fields, not `send_text` in general
    /// (also used for single-keystroke menu selections, where stripping
    /// isn't needed).
    pub fn login(
        &self,
        session: &mut TuiSession,
        id_number: &str,
        access_code: &str,
        ssn_last4: &str,
        birth_date: &str,
    ) -> anyhow::Result<()> {
        for field in Self::sanitize(id_number, access_code, ssn_last4, birth_date) {
            session.send_text(&field)?;
        }
        Ok(())
    }

    fn sanitize(
        id_number: &str,
        access_code: &str,
        ssn_last4: &str,
        birth_date: &str,
    ) -> [String; 4] {
        [id_number, access_code, ssn_last4, birth_date]
            .map(|field| field.chars().filter(|c| !c.is_whitespace()).collect())
    }
}

impl RumadScreen for LoginScreen {
    fn select(&self, _session: &mut TuiSession, _key: &str) -> anyhow::Result<()> {
        anyhow::bail!("Login has no single-keystroke selection; use LoginScreen::login")
    }

    fn line(&self, _session: &mut TuiSession, _text: &str) -> anyhow::Result<()> {
        anyhow::bail!("Login has no free-text prompt; use LoginScreen::login")
    }

    /// PF4 exits the `Login` form specifically -- confirmed live from its
    /// footer ("PF4=(9)"), unlike the "0" every other screen's `exit()`
    /// sends.
    fn exit(&self, session: &mut TuiSession) -> anyhow::Result<()> {
        session.send_key(Key::F4)
    }
}

/// "Ej. 802999999" is unique to the ID-number field's hint and doesn't
/// depend on any accented character, unlike this screen's labels (which
/// arrive mangled -- see `LoginField`'s doc comment). Callers use this to
/// detect the screen; `classify` checks it directly as part of the
/// priority chain.
pub(super) const DETECT_HINT: &str = "Ej. 802999999";

pub(super) fn login_fields() -> Vec<LoginField> {
    vec![
        LoginField {
            key: "id_number".to_string(),
            label: "Número de Identificación".to_string(),
            hint: "Ej. 802999999".to_string(),
        },
        LoginField {
            key: "access_code".to_string(),
            label: "Código de Acceso Permanente".to_string(),
            hint: "Ej. 1234".to_string(),
        },
        LoginField {
            key: "ssn_last4".to_string(),
            label: "Seguro Social (últimos 4)".to_string(),
            hint: "Ej. 1234".to_string(),
        },
        LoginField {
            key: "birth_date".to_string(),
            label: "Fecha de Nacimiento".to_string(),
            hint: "Ej. MMDDAAAA".to_string(),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::LoginScreen;
    use crate::screens::{classify, TuiScreen};

    const LOGIN: &str = include_str!("../../../screens/login.txt");

    #[test]
    fn classifies_login_form() {
        let TuiScreen::Login(LoginScreen { fields }) = classify(LOGIN).screen else {
            panic!("expected Login");
        };
        assert_eq!(
            fields.iter().map(|f| f.key.as_str()).collect::<Vec<_>>(),
            ["id_number", "access_code", "ssn_last4", "birth_date"]
        );
    }

    #[test]
    fn login_strips_whitespace_from_every_field() {
        assert_eq!(
            LoginScreen::sanitize("802 11 1111", "12 34", " 1234", "0101 2000"),
            [
                "802111111".to_string(),
                "1234".to_string(),
                "1234".to_string(),
                "01012000".to_string(),
            ]
        );
    }
}
