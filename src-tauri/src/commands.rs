use std::sync::Mutex;
use tauri::{AppHandle, Manager};

use crate::config;
use crate::screens::{self, TuiScreen};
use crate::ssh::key::Key;
use crate::ssh::session::TuiSession;

/// Holds the one live TUI session for this app instance, if connected.
#[derive(Default)]
pub struct AppState(pub Mutex<Option<TuiSession>>);

/// What a command sends to an already-connected session before reading the
/// resulting screen. `send_input`/`send_text`/`send_key` each used to
/// inline their own copy of the lock-session-act-classify sequence around
/// a differently-shaped `TuiSession` call (`send_line(&str)`,
/// `send_text(&str)`, `send_key(Key)`); unifying the three behind one
/// trait method lets `act` (below) hold that sequence exactly once.
trait SessionInput {
    fn send(self, session: &mut TuiSession) -> anyhow::Result<()>;
}

/// No input at all -- `get_screen` just wants the current screen re-read.
struct NoInput;

impl SessionInput for NoInput {
    fn send(self, _session: &mut TuiSession) -> anyhow::Result<()> {
        Ok(())
    }
}

/// A free-text line terminated by Enter (`send_input`).
struct Line(String);

impl SessionInput for Line {
    fn send(self, session: &mut TuiSession) -> anyhow::Result<()> {
        session.send_line(&self.0)
    }
}

/// Raw text with no trailing Enter (`send_text`).
struct Text(String);

impl SessionInput for Text {
    fn send(self, session: &mut TuiSession) -> anyhow::Result<()> {
        session.send_text(&self.0)
    }
}

/// A single non-printable key (`send_key`).
struct KeyPress(Key);

impl SessionInput for KeyPress {
    fn send(self, session: &mut TuiSession) -> anyhow::Result<()> {
        session.send_key(self.0)
    }
}

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

/// Runs `body` on Tauri's blocking thread pool and flattens the join
/// result. Plain (non-`async fn`) commands run inline on whatever thread
/// dispatches the IPC message -- the GTK main loop on Linux -- so a
/// blocking SSH handshake there would freeze the entire window. Every
/// command below is `async fn` and routes its actual work through here.
async fn blocking<F>(body: F) -> Result<TuiScreen, String>
where
    F: FnOnce() -> Result<TuiScreen, String> + Send + 'static,
{
    tauri::async_runtime::spawn_blocking(body)
        .await
        .map_err(|e| e.to_string())?
}

/// Send `input` to the already-connected session and return the resulting
/// classified screen -- the shared body behind `get_screen`/`send_input`/
/// `send_text`/`send_key`, which differ only in which `SessionInput` they
/// pass in.
async fn act<A>(app: AppHandle, input: A) -> Result<TuiScreen, String>
where
    A: SessionInput + Send + 'static,
{
    blocking(move || {
        let state = app.state::<AppState>();
        let mut guard = state.0.lock().map_err(|_| "session lock poisoned")?;
        let session = guard.as_mut().ok_or("not connected")?;
        input.send(session).map_err(|e| e.to_string())?;
        let raw = session.screen_text();
        log_screen(&raw);
        let screen = screens::classify(&raw);
        finish(&mut guard, screen)
    })
    .await
}

/// Connect over SSH and return the first screen. `username`/`password`
/// default to the shared demo account (see `config`) when omitted, which
/// only ever renders `MENU PRINCIPAL` -- a real student's own credentials
/// are required to reach account-specific screens.
#[tauri::command]
pub async fn connect(
    app: AppHandle,
    username: Option<String>,
    password: Option<String>,
) -> Result<TuiScreen, String> {
    log_invoked(&format!(
        "connect(username={:?}, password={})",
        username,
        if password.is_some() { "<redacted>" } else { "None" }
    ));
    blocking(move || {
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

        let state = app.state::<AppState>();
        let mut guard = state.0.lock().map_err(|_| "session lock poisoned")?;
        *guard = Some(session);
        finish(&mut guard, screen)
    })
    .await
}

/// Re-read the current screen without sending any input, e.g. to poll for
/// a redraw that's still settling.
#[tauri::command]
pub async fn get_screen(app: AppHandle) -> Result<TuiScreen, String> {
    log_invoked("get_screen()");
    act(app, NoInput).await
}

/// Send a line of free text terminated by Enter, then return the
/// resulting screen. For genuinely free-text prompts only (course-code
/// search, `Bajas`/`Altas`/`Cambio`'s "abbreviation or FIN" prompt, etc).
///
/// **Do not use this for numbered/lettered menu selections** -- those
/// screens read a single keystroke with no Enter, confirmed live: sending
/// "5\r" to reach `MENU DESPLIEGUE` from `MENU PRINCIPAL` would send "5"
/// (which navigates immediately) followed by a stray Enter that the
/// freshly-rendered submenu consumes as its own input, bouncing straight
/// back rather than staying on the new screen. Sending "5" alone works
/// every time. Use `send_text` for menu option keys instead.
#[tauri::command]
pub async fn send_input(app: AppHandle, text: String) -> Result<TuiScreen, String> {
    log_invoked(&format!("send_input(text={text:?})"));
    act(app, Line(text)).await
}

/// Send raw text with no trailing Enter. Required for fixed-field forms
/// like `Login`, which explicitly warns against pressing Enter while
/// filling it in, **and** for selecting a numbered/lettered menu option
/// (`MainMenu`, `SelectPeriod`, `Matricula`'s `Actions` prompt, ...) --
/// those screens act on a single keystroke, and appending Enter risks it
/// being consumed as stray input by whatever screen renders next (see
/// `send_input`'s doc comment for the live-verified failure mode). Use
/// `send_input` instead for genuinely free-text prompts that expect a
/// terminated line.
#[tauri::command]
pub async fn send_text(app: AppHandle, text: String) -> Result<TuiScreen, String> {
    log_invoked(&format!("send_text(text={text:?})"));
    act(app, Text(text)).await
}

/// Send a single non-printable key (arrows, Escape, Ctrl-C, ...) by name --
/// see `Key::parse` for the accepted names.
#[tauri::command]
pub async fn send_key(app: AppHandle, key: String) -> Result<TuiScreen, String> {
    log_invoked(&format!("send_key(key={key:?})"));
    let key_value = Key::parse(&key).ok_or_else(|| format!("unknown key {key:?}"))?;
    act(app, KeyPress(key_value)).await
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
