//! Two things live in this crate: the Tauri app (`run`, and `commands` --
//! private, since it's IPC glue built on `tauri::AppHandle`) and a
//! standalone RUMAD-scraping library underneath it (`ssh` to connect and
//! drive a session, `screens` to classify its raw VT100 text into typed
//! data). The two are independent -- nothing in `screens`/`ssh`/`config`
//! depends on Tauri, and the `tauri` feature (on by default, gating
//! `commands`/`run()` plus the `tauri`/`tauri-plugin-opener`/`tauri-build`
//! dependencies themselves) makes that actually enforced rather than just
//! true by convention: a caller that only wants the scraping depends on
//! this crate with `default-features = false` and pulls in none of Tauri.

#[cfg(feature = "tauri")]
mod commands;
pub mod config;
pub mod screens;
pub mod ssh;

#[cfg(feature = "tauri")]
use commands::AppState;

#[cfg(feature = "tauri")]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_google_maps::init())
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            commands::connect,
            commands::get_screen,
            commands::interact::send,
            commands::login::login,
            commands::is_connected,
            commands::disconnect,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
