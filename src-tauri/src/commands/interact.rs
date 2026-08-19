//! The `send` command: the single entry point for every screen
//! interaction that isn't `login`'s own dedicated flow. Replaces what
//! used to be three separate commands (`send_input`, `send_text`,
//! `send_key`), each a raw passthrough straight to `TuiSession` with no
//! idea which screen it was actually acting on. This re-classifies the
//! *current* screen first and dispatches through its `RumadScreen` impl
//! (`TuiScreen::as_rumad_screen`) instead, so what a given keystroke or
//! line of text means is always decided by the screen it's being sent to.

use serde::Deserialize;
use tauri::AppHandle;

use super::exec::{act, log_invoked};
use crate::screens::{self, TuiScreen};

/// Numbered/lettered menu options read a single keystroke with no Enter;
/// free-text prompts (course-code search, `Bajas`/`Altas`/`Cambio`'s
/// "abbreviation or FIN") expect a terminated line; `Exit` is for screens
/// with a dedicated back/leave keystroke that isn't one of their listed
/// options (e.g. `Login`'s PF4). See `RumadScreen`'s doc comment for the
/// live-verified reasoning behind keeping these distinct.
#[derive(Debug, Deserialize)]
#[serde(tag = "kind")]
pub enum SendAction {
    Select { key: String },
    Line { text: String },
    Exit,
}

#[tauri::command]
pub async fn send(app: AppHandle, action: SendAction) -> Result<TuiScreen, String> {
    log_invoked(&format!("send({action:?})"));
    act(app, move |session| {
        let raw = session.screen_text();
        let screen = screens::classify(&raw);
        let rumad_screen = screen
            .as_rumad_screen()
            .ok_or_else(|| anyhow::anyhow!("current screen has no interaction"))?;
        match action {
            SendAction::Select { key } => rumad_screen.select(session, &key),
            SendAction::Line { text } => rumad_screen.line(session, &text),
            SendAction::Exit => rumad_screen.exit(session),
        }
    })
    .await
}
