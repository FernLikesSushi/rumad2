use tauri::{AppHandle, Emitter, Manager};
use tokio::sync::Mutex;

mod exec;

// `pub`, not a `pub use` re-export: `#[tauri::command]` expands to hidden
// sibling items alongside `login`/`send` that `generate_handler!` needs to
// find at their actual path -- a re-export only carries the function name,
// not those macro-generated companions.
pub mod interact;
pub mod login;

use exec::{act, finish, log_invoked, log_screen, spawn_screen_watcher, SCREEN_CHANGED_EVENT};

use crate::config;
use crate::screens::{self, ClassifiedScreen, TuiScreen};
use crate::ssh::session::TuiSession;

/// Holds the one live TUI session for this app instance, if connected.
/// `tokio::sync::Mutex`, not `std::sync::Mutex` -- `act`/`spawn_screen_watcher`
/// hold the lock across `.await` points (the SSH calls themselves), and a
/// std mutex guard held across an await can stall other tasks on the same
/// worker thread rather than just yielding.
#[derive(Default)]
pub struct AppState(pub Mutex<Option<TuiSession>>);

/// Connect over SSH and return the first screen. `username`/`password`
/// default to the shared demo account (see `config`) when omitted, which
/// only ever renders `MENU PRINCIPAL` -- a real student's own credentials
/// are required to reach account-specific screens.
#[tauri::command]
pub async fn connect(
    app: AppHandle,
    username: Option<String>,
    password: Option<String>,
) -> Result<ClassifiedScreen, String> {
    log_invoked(&format!(
        "connect(username={:?}, password={})",
        username,
        if password.is_some() {
            "<redacted>"
        } else {
            "None"
        }
    ));
    let username = username.unwrap_or_else(|| config::DEFAULT_USERNAME.to_string());
    let password = password.unwrap_or_else(|| config::DEFAULT_PASSWORD.to_string());

    let session = TuiSession::connect(
        (config::DEFAULT_HOST, config::SSH_PORT),
        &username,
        &password,
        80,
        24,
    )
    .await
    .map_err(|e| e.to_string())?;
    let raw = session.screen_text();
    log_screen(&raw);
    let result = screens::classify(&raw);

    let state = app.state::<AppState>();
    let mut guard = state.0.lock().await;
    *guard = Some(session);
    let result = finish(&mut guard, result);
    drop(guard);

    // Started here rather than above: needs the session already stored in
    // `AppState` (so its own lock attempts find it) and the resolved
    // screen as its baseline to diff against.
    if let Ok(result) = &result {
        spawn_screen_watcher(app.clone(), result.clone());
    }
    result
}

/// Re-read the current screen without sending any input, e.g. to poll for
/// a redraw that's still settling.
#[tauri::command]
pub async fn get_screen(app: AppHandle) -> Result<ClassifiedScreen, String> {
    log_invoked("get_screen()");
    act(app, async |session| session.refresh().await).await
}

/// Whether a TUI session is currently live -- the frontend uses this to
/// decide where a "go back to the app" link should point (`/` vs
/// `/session`) without tracking connection state itself; `AppState` is the
/// real source of truth for that, not anything the frontend derives on its
/// own from e.g. the current route.
#[tauri::command]
pub async fn is_connected(app: AppHandle) -> Result<bool, String> {
    log_invoked("is_connected()");
    let state = app.state::<AppState>();
    let guard = state.0.lock().await;
    Ok(guard.is_some())
}

/// Log out of the remote menu and drop the session. Emits
/// `SCREEN_CHANGED_EVENT` itself (`spawn_screen_watcher` only sees the
/// channel it's polling go away, not this) so the frontend's one
/// `screen-changed` listener is the single place that reacts to a session
/// ending, regardless of whether that was this explicit logout or the
/// backend's own EOF detection.
#[tauri::command]
pub async fn disconnect(app: AppHandle) -> Result<(), String> {
    log_invoked("disconnect()");
    let state = app.state::<AppState>();
    let mut guard = state.0.lock().await;
    if let Some(session) = guard.take() {
        session.close().await;
    }
    drop(guard);

    let _ = app.emit(
        SCREEN_CHANGED_EVENT,
        &ClassifiedScreen {
            screen: TuiScreen::Disconnected,
            dialog: None,
            can_exit: false,
            can_continue: false,
        },
    );
    Ok(())
}
