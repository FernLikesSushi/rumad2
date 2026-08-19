mod commands;
mod config;
mod screens;
mod ssh;

use commands::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            commands::connect,
            commands::get_screen,
            commands::send_input,
            commands::send_text,
            commands::send_key,
            commands::disconnect,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
