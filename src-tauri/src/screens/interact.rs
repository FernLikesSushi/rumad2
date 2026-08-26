//! `RumadScreen`: the shared interaction trait every screen type
//! implements. Most screens share both interaction shapes this trait
//! models (pick a numbered/lettered option; submit a terminated line of
//! free text); `Login` deviates on entry (its own `login` method, see that
//! type's doc comment) but still exits the same way every other screen
//! does, via `exit()`. `commands::interact`'s `send` command is the one
//! place any of this gets invoked from inside the app itself -- it
//! re-classifies the current screen and dispatches through
//! `TuiScreen::as_rumad_screen` (in `mod.rs`, alongside `TuiScreen`'s
//! other methods) rather than any command sending raw session input
//! directly. `pub`, not `pub(crate)` -- `tests/production.rs` (a real
//! integration test, so it only ever sees this crate's public surface)
//! drives screens through the same trait, against the real production
//! system, rather than duplicating what `select`/`line`/`exit` already do
//! by poking `TuiSession`'s raw primitives itself.
//!
//! `#[async_trait]`, not native `async fn` in the trait: `TuiScreen::
//! as_rumad_screen` hands out `&dyn RumadScreen` (a trait object, since
//! which concrete screen type it's holding varies at runtime), and native
//! `async fn` in traits isn't `dyn`-compatible without boxing the futures
//! itself -- `async_trait` does exactly that under the hood, so every
//! `impl RumadScreen` (all of them `async fn` now, since they call
//! `TuiSession`'s own async methods) needs the same attribute.
//!
//! This file only owns the trait itself -- each screen type's own `impl
//! RumadScreen` lives alongside that screen's struct in its own submodule
//! (`UnknownScreen`'s in `mod.rs`, since it has no submodule of its own),
//! not here.

use async_trait::async_trait;

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
#[async_trait]
pub trait RumadScreen: Sync {
    async fn select(&self, session: &mut TuiSession, key: &str) -> anyhow::Result<()> {
        session.send_text(key).await
    }

    async fn line(&self, session: &mut TuiSession, text: &str) -> anyhow::Result<()> {
        session.send_line(text).await
    }

    /// "Enter to continue" on a read-only screen -- default is a blank
    /// `line`, but named so callers don't send an empty string themselves.
    async fn continue_screen(&self, session: &mut TuiSession) -> anyhow::Result<()> {
        self.line(session, "").await
    }

    /// Go back/exit this screen. Defaults to "0" -- confirmed live as the
    /// consistent back/exit key across both `MainMenu` (0=SALIR DEL
    /// SISTEMA) and `MENU DESPLIEGUE` (0=Finalizar).
    async fn exit(&self, session: &mut TuiSession) -> anyhow::Result<()> {
        self.select(session, "0").await
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

    /// Whether `continue_screen()` actually applies here -- unlike
    /// `can_exit`, defaults to `false` since only read-only "Enter to
    /// continue" screens (`CourseResults`, `WeeklySchedule`) support it.
    fn can_continue(&self) -> bool {
        false
    }
}
