/// Non-printable keys that a real terminal would send as an escape
/// sequence rather than literal bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    Enter,
    Tab,
    Escape,
    Backspace,
    Up,
    Down,
    Left,
    Right,
    Home,
    End,
    CtrlC,
    CtrlD,
    CtrlZ,
    /// Sent by the real remote's `Login` screen's footer as "PF1=(6)".
    F1,
    /// Sent by the real remote's `Login` screen's footer as "PF4=(9)".
    F4,
}

impl Key {
    pub(crate) fn as_bytes(self) -> &'static [u8] {
        match self {
            Key::Enter => b"\r",
            Key::Tab => b"\t",
            Key::Escape => b"\x1b",
            Key::Backspace => b"\x7f",
            Key::Up => b"\x1b[A",
            Key::Down => b"\x1b[B",
            Key::Right => b"\x1b[C",
            Key::Left => b"\x1b[D",
            Key::Home => b"\x1b[H",
            Key::End => b"\x1b[F",
            Key::CtrlC => b"\x03",
            Key::CtrlD => b"\x04",
            Key::CtrlZ => b"\x1a",
            Key::F1 => b"\x1b[17~",
            Key::F4 => b"\x1b[20~",
        }
    }

    pub fn parse(name: &str) -> Option<Key> {
        Some(match name {
            "Enter" => Key::Enter,
            "Tab" => Key::Tab,
            "Escape" => Key::Escape,
            "Backspace" => Key::Backspace,
            "Up" => Key::Up,
            "Down" => Key::Down,
            "Left" => Key::Left,
            "Right" => Key::Right,
            "Home" => Key::Home,
            "End" => Key::End,
            "CtrlC" => Key::CtrlC,
            "CtrlD" => Key::CtrlD,
            "CtrlZ" => Key::CtrlZ,
            "F6" => Key::F1,
            "F9" => Key::F4,
            _ => return None,
        })
    }
}
