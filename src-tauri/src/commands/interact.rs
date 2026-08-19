//! The `send` command: the single entry point for every screen
//! interaction that isn't `login`'s own dedicated flow. Replaces what
//! used to be three separate commands (`send_input`, `send_text`,
//! `send_key`), each a raw passthrough straight to `TuiSession` with no
//! idea which screen it was actually acting on. This re-classifies the
//! *current* screen first and dispatches through its `RumadScreen` impl
//! (`TuiScreen::as_rumad_screen`) instead, so what a given keystroke or
//! line of text means is always decided by the screen it's being sent to.
//! Any `Dialog` overlaid on that screen doesn't change what's dispatched
//! -- `screens::classify`'s `screen` half is always the live, interactive
//! screen regardless of whether a dialog is also showing on top of it, so
//! there's no special-casing needed here for that (unlike the old design,
//! where a detected notice used to *replace* the screen and had to be
//! passed through as a plain send instead).

use serde::Deserialize;
use tauri::AppHandle;

use super::exec::{act, log_invoked};
use crate::screens::{self, ClassifiedScreen};

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
pub async fn send(app: AppHandle, action: SendAction) -> Result<ClassifiedScreen, String> {
    log_invoked(&format!("send({action:?})"));
    act(app, move |session| {
        let raw = session.screen_text();
        let screen = screens::classify(&raw).screen;
        match screen.as_rumad_screen() {
            Some(rumad_screen) => match action {
                SendAction::Select { key } => rumad_screen.select(session, &key),
                SendAction::Line { text } => rumad_screen.line(session, &text),
                SendAction::Exit => rumad_screen.exit(session),
            },
            None => anyhow::bail!("current screen has no interaction"),
        }
    })
    .await
}
