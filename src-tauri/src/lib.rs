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
use tauri::Manager;

// WKWebView's own scroll view already rubber-bands once its content
// overflows, but doesn't `alwaysBounceVertical` -- so a screen whose
// content exactly fills the viewport (nothing to overflow) has no
// bounce at all, unlike a real iOS app (Settings.app, Mail, ...),
// which always gives a little give on pull even then. There's no
// `tauri.conf.json` setting for this; it's a raw Objective-C message
// send against the `UIScrollView` Tauri's iOS runtime hands back via
// `Webview::with_webview` -- same "reach into the native view
// hierarchy" pattern `tauri-plugin-native-map`'s Swift plugin already
// uses for `MKMapView`, just from the Rust side instead, since this
// only needs one property flipped rather than a whole plugin.
#[cfg(all(feature = "tauri", target_os = "ios"))]
fn enable_native_bounce(window: &tauri::WebviewWindow) {
    use objc2::msg_send;
    use objc2::runtime::AnyObject;

    let _ = window.with_webview(|webview| {
        // SAFETY: `inner()` is the live `WKWebView*` for this window,
        // valid for the closure's duration; `with_webview` runs it on
        // the main thread, same as every other UIKit/WebKit call site
        // in this app (see `NativeMapPlugin.swift`'s doc comments).
        unsafe {
            let webview: *mut AnyObject = webview.inner().cast();
            let scroll_view: *mut AnyObject = msg_send![webview, scrollView];
            let _: () = msg_send![scroll_view, setBounces: true];
            let _: () = msg_send![scroll_view, setAlwaysBounceVertical: true];
        }
    });
}

#[cfg(feature = "tauri")]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_native_map::init())
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            commands::connect,
            commands::get_screen,
            commands::interact::send,
            commands::login::login,
            commands::is_connected,
            commands::disconnect,
        ])
        .setup(|_app| {
            #[cfg(target_os = "ios")]
            if let Some(window) = _app.get_webview_window("main") {
                enable_native_bounce(&window);
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
