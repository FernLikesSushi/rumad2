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
    CourseResultsScreen, LoginScreen, MatriculaMode, MatriculaScreen, MenuKind, MenuScreen,
    SearchScreen, UnknownScreen, WeeklyScheduleScreen,
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

    /// Whether `exit()` actually goes anywhere, rather than just bailing.
    /// A screen whose exit key also happens to be listed among its own
    /// `options` (`MainMenu`'s "0", `SelectPeriod`'s "S=salir") still
    /// counts as `true` here -- the frontend's dedicated exit control
    /// (see `ClassifiedScreen::can_exit`) is deliberately the *one*
    /// consistent, discoverable way to leave any screen, the same role
    /// PF4 plays on screens that use it, rather than making the user hunt
    /// for "0" among a wall of option buttons on the ones that don't.
    /// Only screens whose `exit()` genuinely has nowhere to send (see
    /// `MatriculaScreen`/`WeeklyScheduleScreen`'s own overrides) return
    /// `false`.
    fn can_exit(&self) -> bool {
        true
    }
}

impl RumadScreen for UnknownScreen {}

impl RumadScreen for MenuScreen {
    /// `MainMenu`/`MenuDespliegue`/`SelectPeriod` all have a free-text
    /// prompt available (even if unused) via the default; `HorarioSemester`
    /// genuinely has none, just its four numbered/lettered options.
    fn line(&self, session: &mut TuiSession, text: &str) -> anyhow::Result<()> {
        if self.menu == MenuKind::HorarioSemester {
            anyhow::bail!("HorarioSemester has no free-text prompt; use select()")
        }
        session.send_line(text)
    }

    /// `MainMenu`/`MenuDespliegue`/`SelectPeriod` all exit via the default
    /// "0" (confirmed live for the first two; `SelectPeriod`'s own
    /// "S=salir" is just a normal `select`, not a separate exit keystroke,
    /// so the default "0" is simply never used there but is harmless).
    /// `HorarioSemester` is the one exception -- PF4 exits it specifically,
    /// per its own footer's "[PF4=(9)Fin]" hint. Either way `exit()`
    /// reaches a real destination for every kind, so `can_exit`'s default
    /// (`true`) needs no override here -- see that method's own doc
    /// comment on why this is deliberately still worth a dedicated
    /// control even for the three kinds whose exit key is also listed
    /// among `options`.
    fn exit(&self, session: &mut TuiSession) -> anyhow::Result<()> {
        match self.menu {
            MenuKind::HorarioSemester => session.send_key(Key::F4),
            _ => self.select(session, "0"),
        }
    }
}

impl RumadScreen for SearchScreen {
    /// No numbered options on either kind, just the free-text search.
    fn select(&self, _session: &mut TuiSession, _key: &str) -> anyhow::Result<()> {
        anyhow::bail!("Search has no selectable options; use line()")
    }

    /// PF4 exits both kinds specifically -- their own footer's
    /// "[PF4=(9)Fin]" hint.
    fn exit(&self, session: &mut TuiSession) -> anyhow::Result<()> {
        session.send_key(Key::F4)
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

impl RumadScreen for MatriculaScreen {
    /// `Actions` exits via its own `S`=Salir option -- a normal `select`,
    /// same as `MenuScreen`'s "0"/"S=salir" (see that impl's `exit` doc
    /// comment for why that still counts as a real exit destination, not
    /// a bail). `Bajas`/`Altas`/`Cambio` have no such listed option --
    /// they exit via the free-text "FIN" instead.
    fn exit(&self, session: &mut TuiSession) -> anyhow::Result<()> {
        match self.mode {
            MatriculaMode::Actions { .. } => self.select(session, "S"),
            MatriculaMode::Bajas | MatriculaMode::Altas | MatriculaMode::Cambio => {
                self.line(session, "FIN")
            }
        }
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
        anyhow::bail!(
            "WeeklySchedule's exit keystroke isn't confirmed live; try line(\"\") instead"
        )
    }

    fn can_exit(&self) -> bool {
        false
    }
}
