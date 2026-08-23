// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// The `rumad-2` binary *is* the Tauri app -- library-only consumers should
// depend on this crate with `default-features = false` and use its
// `screens`/`ssh`/`config` modules directly instead of building this bin.
#[cfg(not(feature = "tauri"))]
compile_error!(
    "the rumad-2 binary requires the `tauri` feature (enabled by default) -- build with `--no-default-features --lib` for the scraping library alone"
);

fn main() {
    rumad_2_lib::run()
}
