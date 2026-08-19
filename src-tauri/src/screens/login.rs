//! The real per-student authentication form (ID number, permanent access
//! code, last 4 of SSN, birth date) shown after selecting "Seleccion de
//! Secciones" -- separate from and unrelated to the SSH login, which every
//! student shares (`estudiante@...`).

use serde::Serialize;

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
    use crate::screens::{classify, TuiScreen};

    const LOGIN: &str = include_str!("../../../screens/login.txt");

    #[test]
    fn classifies_login_form() {
        let TuiScreen::Login { fields } = classify(LOGIN) else {
            panic!("expected Login");
        };
        assert_eq!(
            fields.iter().map(|f| f.key.as_str()).collect::<Vec<_>>(),
            ["id_number", "access_code", "ssn_last4", "birth_date"]
        );
    }
}
