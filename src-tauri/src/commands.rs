use std::sync::Mutex;
use tauri::State;

use crate::config;
use crate::screens::{self, TuiScreen};
use crate::ssh::key::Key;
use crate::ssh::session::TuiSession;

/// Holds the one live TUI session for this app instance, if connected.
#[derive(Default)]
pub struct AppState(pub Mutex<Option<TuiSession>>);

/// Finish a command: if the screen signals a graceful end of session
/// (`Disconnected` -- the remote closes the channel shortly after showing
/// it), drop the session so later commands correctly report "not
/// connected" instead of erroring on a dead channel. Otherwise apply
/// `or_err` to promote known failure notices to an `Err`.
fn finish(guard: &mut Option<TuiSession>, screen: TuiScreen) -> Result<TuiScreen, String> {
    if screen == TuiScreen::Disconnected {
        *guard = None;
    }
    screen.or_err()
}

/// Logs every command invocation and the raw remote screen after it runs,
/// so `cargo tauri dev`'s stdout shows what the TUI actually looks like
/// alongside what triggered each change.
fn log_invoked(command: &str) {
    println!("[tui] > {command}");
}

fn log_screen(raw: &str) {
    println!("[tui] screen:\n{raw}\n[tui] --- end screen ---");
}

/// Connect over SSH and return the first screen. `username`/`password`
/// default to the shared demo account (see `config`) when omitted, which
/// only ever renders `MENU PRINCIPAL` -- a real student's own credentials
/// are required to reach account-specific screens.
#[tauri::command]
pub fn connect(
    state: State<AppState>,
    username: Option<String>,
    password: Option<String>,
) -> Result<TuiScreen, String> {
    log_invoked(&format!(
        "connect(username={:?}, password={})",
        username,
        if password.is_some() { "<redacted>" } else { "None" }
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
    .map_err(|e| e.to_string())?;
    let raw = session.screen_text();
    log_screen(&raw);
    let screen = screens::classify(&raw);

    let mut guard = state.0.lock().map_err(|_| "session lock poisoned")?;
    *guard = Some(session);
    finish(&mut guard, screen)
}

/// Re-read the current screen without sending any input, e.g. to poll for
/// a redraw that's still settling.
#[tauri::command]
pub fn get_screen(state: State<AppState>) -> Result<TuiScreen, String> {
    log_invoked("get_screen()");
    let mut guard = state.0.lock().map_err(|_| "session lock poisoned")?;
    let session = guard.as_ref().ok_or("not connected")?;
    let raw = session.screen_text();
    log_screen(&raw);
    let screen = screens::classify(&raw);
    finish(&mut guard, screen)
}

/// Send a menu choice or free-text input line, then return the resulting
/// screen. Used both for numbered menu options ("2") and for any screen
/// that expects a typed value followed by Enter.
#[tauri::command]
pub fn send_input(state: State<AppState>, text: String) -> Result<TuiScreen, String> {
    log_invoked(&format!("send_input(text={text:?})"));
    let mut guard = state.0.lock().map_err(|_| "session lock poisoned")?;
    let session = guard.as_mut().ok_or("not connected")?;
    session.send_line(&text).map_err(|e| e.to_string())?;
    let raw = session.screen_text();
    log_screen(&raw);
    let screen = screens::classify(&raw);
    finish(&mut guard, screen)
}

/// Send a single non-printable key (arrows, Escape, Ctrl-C, ...) by name --
/// see `Key::parse` for the accepted names.
#[tauri::command]
pub fn send_key(state: State<AppState>, key: String) -> Result<TuiScreen, String> {
    log_invoked(&format!("send_key(key={key:?})"));
    let key_value = Key::parse(&key).ok_or_else(|| format!("unknown key {key:?}"))?;
    let mut guard = state.0.lock().map_err(|_| "session lock poisoned")?;
    let session = guard.as_mut().ok_or("not connected")?;
    session.send_key(key_value).map_err(|e| e.to_string())?;
    let raw = session.screen_text();
    log_screen(&raw);
    let screen = screens::classify(&raw);
    finish(&mut guard, screen)
}

/// Log out of the remote menu and drop the session.
#[tauri::command]
pub fn disconnect(state: State<AppState>) -> Result<(), String> {
    log_invoked("disconnect()");
    let mut guard = state.0.lock().map_err(|_| "session lock poisoned")?;
    if let Some(session) = guard.take() {
        session.close();
    }
    Ok(())
}
