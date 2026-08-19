//! Execution plumbing shared by every command: running blocking SSH work
//! off the IPC thread, sending input to the session and turning the
//! resulting screen text into a classified `TuiScreen`, and the stdout
//! logging that mirrors both onto the terminal.

use std::thread;
use std::time::Duration;
use tauri::{AppHandle, Emitter, Manager};

use super::AppState;
use crate::screens::{self, TuiScreen};
use crate::ssh::session::TuiSession;

/// Event name the frontend listens for (`@tauri-apps/api/event`'s
/// `listen`) to redraw itself when `spawn_screen_watcher` sees a change --
/// see that function's doc comment.
const SCREEN_CHANGED_EVENT: &str = "screen-changed";

/// Finish a command: if the screen signals a graceful end of session
/// (`Disconnected` -- the remote closes the channel shortly after showing
/// it), drop the session so later commands correctly report "not
/// connected" instead of erroring on a dead channel. Otherwise apply
/// `or_err` to promote known failure notices to an `Err`.
pub(super) fn finish(
    guard: &mut Option<TuiSession>,
    screen: TuiScreen,
) -> Result<TuiScreen, String> {
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

fn handle_scene_change(guard: &mut Option<TuiSession>) -> Result<TuiScreen, std::string::String> {
    let Some(session) = guard else {
        return Err("not connected".to_string());
    };

    let raw = session.screen_text();
    log_screen(&raw);
    let screen = screens::classify(&raw);
    finish(guard, screen)
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
        handle_scene_change(&mut guard)
    })
    .await
}

/// Long-lived background reader, one per connected session (started right
/// after `connect` stores it): ticks continuously, draining whatever's
/// arrived on the channel and emitting `SCREEN_CHANGED_EVENT` whenever the
/// classified screen actually changes. Exists because the remote isn't
/// always driven by a request -- some screens redraw a second time on
/// their own after computing a result server-side (see
/// `TuiScreen::Processing`), and nothing else is watching for that once
/// the command that triggered it has already returned. Exits once the
/// session is gone (`disconnect`, or the remote's own "PROCESO
/// CONCLUIDO"). Runs independently of `act`'s own foreground reads --
/// both just take turns through the same `AppState` lock.
pub(super) fn spawn_screen_watcher(app: AppHandle, mut last: TuiScreen) {
    thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(200));

        let state = app.state::<AppState>();
        let Ok(mut guard) = state.0.lock() else {
            return;
        };
        let Some(session) = guard.as_mut() else {
            return;
        };
        let changed = match session.drain_available() {
            Ok(changed) => changed,
            Err(_) => return,
        };
        let screen = screens::classify(&session.screen_text());
        
        if changed && screen != last {
            handle_scene_change(&mut guard).ok();
            drop(guard);
            
            let _ = app.emit(SCREEN_CHANGED_EVENT, &screen);
            last = screen;
        }
    });
}
