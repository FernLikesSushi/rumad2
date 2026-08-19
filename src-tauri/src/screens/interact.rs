//! Per-screen-type interaction. Most screens share both interaction shapes
//! this trait models (pick a numbered/lettered option; submit a
//! terminated line of free text); `Login` deviates on entry (its own
//! `login` method, see that type's doc comment) but still exits the same
//! way every other screen does, via `exit()`. `commands::interact`'s
//! `send` command is the single place any of this actually gets invoked
//! from -- it re-classifies the current screen and dispatches through
//! `TuiScreen::as_rumad_screen` (in `mod.rs`, alongside `TuiScreen`'s
//! other methods) rather than any command sending raw session input
//! directly.

use super::{
    CourseResultsScreen, LoginScreen, MainMenuScreen, MatriculaScreen, SelectPeriodScreen,
    UnknownScreen, WeeklyScheduleScreen,
};
use crate::ssh::key::Key;
use crate::ssh::session::TuiSession;

/// Screens with either or both of: picking a numbered/lettered option by
/// a single keystroke, or submitting a terminated line of free text.
/// Confirmed live: option screens read one keystroke with no Enter --
/// appending one risks it being consumed as stray input by whatever
/// screen renders next, which is why `select` and `line` stay distinct
/// rather than one method that always appends Enter -- while free-text
/// prompts (course-code search, `Bajas`/`Altas`/`Cambio`'s "abbreviation
/// or FIN") expect a terminated line. Screens that deviate on entry (e.g.
/// `LoginScreen::login`) still implement this trait for `exit()`,
/// overriding `select`/`line`'s defaults if they don't support that shape
/// of interaction at all.
pub(crate) trait RumadScreen {
    fn select(&self, session: &mut TuiSession, key: &str) -> anyhow::Result<()> {
        session.send_text(key)
    }

    fn line(&self, session: &mut TuiSession, text: &str) -> anyhow::Result<()> {
        session.send_line(text)
    }

    /// Go back/exit this screen. Defaults to "0" -- confirmed live as the
    /// consistent back/exit key across both `MainMenu` (0=SALIR DEL
    /// SISTEMA) and `MENU DESPLIEGUE` (0=Finalizar).
    fn exit(&self, session: &mut TuiSession) -> anyhow::Result<()> {
        self.select(session, "0")
    }
}

impl RumadScreen for MainMenuScreen {}
impl RumadScreen for SelectPeriodScreen {}
impl RumadScreen for UnknownScreen {}

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

impl RumadScreen for MatriculaScreen {
    /// Unlike `MainMenu`/`MENU DESPLIEGUE`, `Matricula`'s `Actions` prompt
    /// has no "0" option (its own `S`=Salir is just a normal `select`, not
    /// a separate exit keystroke) and its `Bajas`/`Altas`/`Cambio`
    /// sub-prompts exit via the free-text "FIN" instead. There's no single
    /// keystroke that works across both shapes, so this is left
    /// unimplemented rather than guessing -- callers should `select`/
    /// `line` the screen's own documented way out instead.
    fn exit(&self, _session: &mut TuiSession) -> anyhow::Result<()> {
        anyhow::bail!("Matricula has no single exit keystroke; select \"S\" or send the line \"FIN\" instead")
    }
}

impl RumadScreen for CourseResultsScreen {
    /// No numbered options on this screen, just "Enter to continue" (the
    /// `line` default already sends a bare Enter) or PF4 to leave.
    fn select(&self, _session: &mut TuiSession, _key: &str) -> anyhow::Result<()> {
        anyhow::bail!("CourseResults has no selectable options; use line(\"\") or exit()")
    }

    /// PF4 exits this screen specifically -- confirmed live from its own
    /// footer ("< Oprima Enter o [PF4(9)=Fin] >").
    fn exit(&self, session: &mut TuiSession) -> anyhow::Result<()> {
        session.send_key(Key::F4)
    }
}

impl RumadScreen for WeeklyScheduleScreen {
    /// No numbered options on this read-only grid.
    fn select(&self, _session: &mut TuiSession, _key: &str) -> anyhow::Result<()> {
        anyhow::bail!("WeeklySchedule has no selectable options; use line(\"\") instead")
    }

    /// Unlike `CourseResults`, the real capture of this screen has no
    /// visible footer/prompt line to confirm an exit keystroke against
    /// (see `weekly_schedule.rs`'s doc comment on that transcript's
    /// capture issues) -- left unimplemented rather than guessing. `line`
    /// (bare Enter, inherited default) is a safe bet since every other
    /// screen in this app accepts it to continue/redraw.
    fn exit(&self, _session: &mut TuiSession) -> anyhow::Result<()> {
        anyhow::bail!("WeeklySchedule's exit keystroke isn't confirmed live; try line(\"\") instead")
    }
}
