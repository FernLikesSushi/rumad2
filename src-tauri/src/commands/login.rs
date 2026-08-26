//! The `login` command: fills in the real per-student `Login` form's four
//! fields. Confirms the session is actually showing that screen before
//! sending anything -- `LoginScreen::login` (its own concern, see that
//! doc comment) does the actual sanitized sends.

use tauri::AppHandle;

use super::exec::{act, log_invoked};
use crate::screens::{self, ClassifiedScreen, TuiScreen};

#[tauri::command]
pub async fn login(
    app: AppHandle,
    id_number: String,
    access_code: String,
    ssn_last4: String,
    birth_date: String,
) -> Result<ClassifiedScreen, String> {
    log_invoked(&format!(
        "login(id_number={id_number:?}, access_code=<redacted>, ssn_last4=<redacted>, birth_date={birth_date:?})"
    ));
    act(app, async move |session| {
        let raw = session.screen_text();
        let TuiScreen::Login(login_screen) = screens::classify(&raw).screen else {
            anyhow::bail!("not on the Login screen");
        };
        login_screen
            .login(session, &id_number, &access_code, &ssn_last4, &birth_date)
            .await
    })
    .await
}
