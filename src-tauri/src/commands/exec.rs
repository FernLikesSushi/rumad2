//! Execution plumbing shared by every command: running blocking SSH work
//! off the IPC thread, sending input to the session and turning the
//! resulting screen text into a classified `TuiScreen`, and the stdout
//! logging that mirrors both onto the terminal.

use tauri::{AppHandle, Manager};

use super::AppState;
use crate::screens::{self, TuiScreen};
use crate::ssh::session::TuiSession;

/// Finish a command: if the screen signals a graceful end of session
/// (`Disconnected` -- the remote closes the channel shortly after showing
/// it), drop the session so later commands correctly report "not
/// connected" instead of erroring on a dead channel. Otherwise apply
/// `or_err` to promote known failure notices to an `Err`.
pub(super) fn finish(guard: &mut Option<TuiSession>, screen: TuiScreen) -> Result<TuiScreen, String> {
    if screen == TuiScreen::Disconnected {
        *guard = None;
    }
    screen.or_err()
}

/// Logs every command invocation and the raw remote screen after it runs,
/// so `cargo tauri dev`'s stdout shows what the TUI actually looks like
/// alongside what triggered each change.
pub(super) fn log_invoked(command: &str) {
    println!("[tui] > {command}");
}

pub(super) fn log_screen(raw: &str) {
    println!("[tui] screen:\n{raw}\n[tui] --- end screen ---");
}

/// Runs `body` on Tauri's blocking thread pool and flattens the join
/// result. Plain (non-`async fn`) commands run inline on whatever thread
/// dispatches the IPC message -- the GTK main loop on Linux -- so a
/// blocking SSH handshake there would freeze the entire window. Every
/// command in `mod.rs` is `async fn` and routes its actual work through
/// here (directly, or via `act` below).
pub(super) async fn blocking<F>(body: F) -> Result<TuiScreen, String>
where
    F: FnOnce() -> Result<TuiScreen, String> + Send + 'static,
{
    tauri::async_runtime::spawn_blocking(body)
        .await
        .map_err(|e| e.to_string())?
}

/// Runs `send` against the already-connected session and returns the
/// resulting classified screen -- the shared body behind `get_screen`,
/// `interact::send`, and `login`, which differ only in what `send` does
/// (nothing, dispatch through the current screen's `RumadScreen` impl, or
/// a sequence of `Login` fields).
pub(super) async fn act<F>(app: AppHandle, send: F) -> Result<TuiScreen, String>
where
    F: FnOnce(&mut TuiSession) -> anyhow::Result<()> + Send + 'static,
{
    blocking(move || {
        let state = app.state::<AppState>();
        let mut guard = state.0.lock().map_err(|_| "session lock poisoned")?;
        let session = guard.as_mut().ok_or("not connected")?;
        send(session).map_err(|e| e.to_string())?;
        let raw = session.screen_text();
        log_screen(&raw);
        let screen = screens::classify(&raw);
        finish(&mut guard, screen)
    })
    .await
}
