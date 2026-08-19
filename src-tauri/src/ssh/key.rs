/// Non-printable keys that a real terminal would send as an escape
/// sequence rather than literal bytes. Trimmed to just what's actually
/// constructed somewhere in the codebase -- add a variant back grounded
/// in a specific, confirmed-live need rather than pre-emptively.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    Enter,
    /// Sent by the real remote's `Login` screen's footer as "PF4=(9)";
    /// exits that screen (see `LoginScreen`'s `RumadScreen::exit`).
    F4,
}

impl Key {
    pub(crate) fn as_bytes(self) -> &'static [u8] {
        match self {
            Key::Enter => b"\r",
            Key::F4 => b"\x1b[20~",
        }
    }
}
