//! Two things live in this crate: the Tauri app (`run`, and `commands` --
//! private, since it's IPC glue built on `tauri::AppHandle`) and a
//! standalone RUMAD-scraping library underneath it (`ssh` to connect and
//! drive a session, `screens` to classify its raw VT100 text into typed
//! data). The two are independent -- nothing in `screens`/`ssh`/`config`
//! depends on Tauri, so a caller that only wants the scraping can depend on
//! this crate and use those modules directly without pulling in a GUI.

mod commands;
pub mod config;
pub mod screens;
pub mod ssh;

use commands::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            commands::connect,
            commands::get_screen,
            commands::interact::send,
            commands::login::login,
            commands::disconnect,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
