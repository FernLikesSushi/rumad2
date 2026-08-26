//! Execution plumbing shared by every command: sending input to the
//! session and turning the resulting screen text into a classified
//! `ClassifiedScreen`, and the stdout logging that mirrors both onto the
//! terminal.

use std::time::Duration;
use tauri::{AppHandle, Emitter, Manager};

use super::AppState;
use crate::screens::{self, ClassifiedScreen, TuiScreen};
use crate::ssh::session::TuiSession;

/// Event name the frontend listens for (`@tauri-apps/api/event`'s
/// `listen`) to redraw itself when `spawn_screen_watcher` sees a change --
/// see that function's doc comment. `pub(super)` so `mod.rs`'s `disconnect`
/// command can emit it too, for the same reason `handle_scene_change`
/// constructs `Disconnected` here: one event, whether the session ends
/// because the backend saw EOF or because the user explicitly logged out.
pub(super) const SCREEN_CHANGED_EVENT: &str = "screen-changed";

/// If the screen signals a graceful end of session (`Disconnected` --
/// `handle_scene_change` below has already confirmed the SSH channel
/// actually closed), drop the session so later commands correctly report
/// "not connected" instead of erroring on a dead channel. A real side
/// effect on `AppState`, so this runs regardless of whether the caller
/// wants `or_err`'s `Err`-promotion too (`finish` does; `spawn_screen_watcher`
/// doesn't -- an `or_err`-promoted rejection is a normal thing to observe
/// mid-poll, not a reason to stop watching).
fn clear_if_disconnected(guard: &mut Option<TuiSession>, result: &ClassifiedScreen) {
    if result.screen == TuiScreen::Disconnected {
        *guard = None;
    }
}

/// Finish a command: applies `clear_if_disconnected`, then `or_err` to
/// promote known failure notices to an `Err`.
pub(super) fn finish(
    guard: &mut Option<TuiSession>,
    result: ClassifiedScreen,
) -> Result<ClassifiedScreen, String> {
    clear_if_disconnected(guard, &result);
    result.or_err()
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

/// Re-reads and re-classifies the current screen, applying
/// `clear_if_disconnected`'s side effect -- the shared tail end of every
/// foreground read (`act`) and every background tick
/// (`spawn_screen_watcher`). Deliberately doesn't apply `or_err`'s
/// `Err`-promotion itself: `act` wants that for its command-facing
/// result, but `spawn_screen_watcher` doesn't -- an `or_err`-promoted
/// rejection is a normal thing to observe mid-poll, not a reason to stop
/// watching.
///
/// `screens::classify` is text-only and, deliberately, never returns
/// `Disconnected` on its own (see that function's doc comment) -- the
/// remote's "PROCESO CONCLUIDO" banner can show up without the SSH
/// channel actually closing. This is the one place that overrides to
/// `Disconnected`, and only once `TuiSession::is_closed()` confirms the
/// channel has genuinely reached EOF (always called right after a pump,
/// so that's freshly accurate here).
fn handle_scene_change(guard: &mut Option<TuiSession>) -> Result<ClassifiedScreen, String> {
    let Some(session) = guard else {
        return Err("not connected".to_string());
    };

    let raw = session.screen_text();
    log_screen(&raw);
    let mut result = screens::classify(&raw);
    if session.is_closed() {
        result = ClassifiedScreen {
            screen: TuiScreen::Disconnected,
            dialog: None,
            can_exit: false,
            can_continue: false,
        };
    }
    clear_if_disconnected(guard, &result);
    Ok(result)
}

/// Runs `send` against the already-connected session and returns the
/// resulting classified screen -- the shared body behind `get_screen`,
/// `interact::send`, and `login`, which differ only in what `send` does
/// (nothing, dispatch through the current screen's `RumadScreen` impl, or
/// a sequence of `Login` fields). `russh`, this app's SSH library, is
/// async-only, so `TuiSession`/`RumadScreen` are async too; Tauri's
/// command handlers already run on its own async runtime, so an `async
/// fn` command that `.await`s here needs no dedicated thread of its own.
/// `AsyncFnOnce`, not a plain `FnOnce(&mut TuiSession) -> impl Future` --
/// callers need to borrow `session` themselves (e.g. to classify the
/// current screen and dispatch through it), and only a real async closure
/// lets that borrow's lifetime track the argument correctly; a
/// manually-split `Fut` generic can't express that dependency without
/// boxing the future.
pub(super) async fn act<F>(app: AppHandle, send: F) -> Result<ClassifiedScreen, String>
where
    F: AsyncFnOnce(&mut TuiSession) -> anyhow::Result<()>,
{
    let state = app.state::<AppState>();
    let mut guard = state.0.lock().await;
    let session = guard.as_mut().ok_or("not connected")?;
    send(session).await.map_err(|e| e.to_string())?;
    handle_scene_change(&mut guard)?.or_err()
}

/// Long-lived background reader, one per connected session (started right
/// after `connect` stores it): ticks continuously, draining whatever's
/// arrived on the channel and emitting `SCREEN_CHANGED_EVENT` whenever the
/// classified result actually changes (screen, dialog, or both). Exists
/// because the remote isn't always driven by a request -- some screens
/// redraw a second time on their own after computing a result
/// server-side (see `Dialog::Processing`), and nothing else is watching
/// for that once the command that triggered it has already returned.
/// Exits once the session is gone (`disconnect`, or the SSH channel
/// actually closing -- see `handle_scene_change`). Runs independently of
/// `act`'s own foreground reads -- both just take turns through the same
/// `AppState` lock. A `tokio::spawn`ed task, not a plain OS thread --
/// `TuiSession::drain_available` is async, and `tokio::time::sleep`
/// yields the runtime between ticks instead of blocking a whole thread.
pub(super) fn spawn_screen_watcher(app: AppHandle, mut last: ClassifiedScreen) {
    tauri::async_runtime::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_millis(200)).await;

            let state = app.state::<AppState>();
            let mut guard = state.0.lock().await;
            let Some(session) = guard.as_mut() else {
                return;
            };
            let changed = match session.drain_available().await {
                Ok(changed) => changed,
                Err(_) => return,
            };
            if !changed {
                continue;
            }
            let Ok(result) = handle_scene_change(&mut guard) else {
                return;
            };
            drop(guard);

            if result != last {
                let _ = app.emit(SCREEN_CHANGED_EVENT, &result);
                last = result;
            }
        }
    });
}
