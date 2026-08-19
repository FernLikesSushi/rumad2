use std::sync::Mutex;
use tauri::{AppHandle, Manager};

mod exec;

// `pub`, not a `pub use` re-export: `#[tauri::command]` expands to hidden
// sibling items alongside `login`/`send` that `generate_handler!` needs to
// find at their actual path -- a re-export only carries the function name,
// not those macro-generated companions.
pub mod interact;
pub mod login;

use exec::{act, blocking, finish, log_invoked, log_screen, spawn_screen_watcher};

use crate::config;
use crate::screens::{self, ClassifiedScreen};
use crate::ssh::session::TuiSession;

/// Holds the one live TUI session for this app instance, if connected.
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
    let watcher_app = app.clone();
    let result = blocking(move || {
        let username = username.unwrap_or_else(|| config::DEFAULT_USERNAME.to_string());
        let password = password.unwrap_or_else(|| config::DEFAULT_PASSWORD.to_string());

        let session = TuiSession::connect(
            (config::DEFAULT_HOST, config::SSH_PORT),
            &username,
            &password,
            80,
            24,
        )
        .map_err(|e| e.to_string())?;
        let raw = session.screen_text();
        log_screen(&raw);
        let result = screens::classify(&raw);

        let state = app.state::<AppState>();
        let mut guard = state.0.lock().map_err(|_| "session lock poisoned")?;
        *guard = Some(session);
        finish(&mut guard, result)
    })
    .await;

    // Started here rather than inside the closure above: needs the
    // session already stored in `AppState` (so its own lock attempts find
    // it) and the resolved screen as its baseline to diff against.
    if let Ok(result) = &result {
        spawn_screen_watcher(watcher_app, result.clone());
    }
    result
}

/// Re-read the current screen without sending any input, e.g. to poll for
/// a redraw that's still settling.
#[tauri::command]
pub async fn get_screen(app: AppHandle) -> Result<ClassifiedScreen, String> {
    log_invoked("get_screen()");
    act(app, |session| session.refresh()).await
}

/// Log out of the remote menu and drop the session.
#[tauri::command]
pub async fn disconnect(app: AppHandle) -> Result<(), String> {
    log_invoked("disconnect()");
    tauri::async_runtime::spawn_blocking(move || {
        let state = app.state::<AppState>();
        let mut guard = state.0.lock().map_err(|_| "session lock poisoned")?;
        if let Some(session) = guard.take() {
            session.close();
        }
        Ok(())
    })
    .await
    .map_err(|e| e.to_string())?
}
