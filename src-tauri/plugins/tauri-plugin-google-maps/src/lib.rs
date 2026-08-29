//! App-internal Tauri plugin embedding a native Android `MapView`
//! (`com.google.android.gms.maps`) alongside the webview, since a plain
//! `<iframe>` (the `GoogleMapWeb` component) has no native-map
//! affordances (gestures, markers, my-location) and looks noticeably
//! worse than a real map surface on a phone. Desktop and iOS have no
//! native view here -- `desktop.rs` covers both, returning `Error::
//! Unsupported` for every command, and the frontend falls back to
//! `GoogleMapWeb` there instead (see `GoogleMapNative.tsx`).
//!
//! The native `MapView` isn't part of the DOM -- it's a separate Android
//! `View` layered on top of the WebView's own view, positioned to match
//! a placeholder element's `getBoundingClientRect()` (see `Frame` in
//! `models.rs`). `create_map`/`update_frame` reposition it; `set_camera`/
//! `set_marker` change what it's showing without moving it; `dispose`
//! tears it down (called on the frontend component's `onCleanup`).

// A Tauri "plugin" is just a bundle of: some `#[tauri::command]`
// functions the frontend can call via `invoke()`, plus optional setup
// code that runs once when the app starts. `tauri_plugin_opener::init()`
// (registered right before this one in `src-tauri/src/lib.rs`) is the
// exact same shape as `init()` below -- this file is what's *inside* one
// of those `init()` calls.
use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

// Everything here is compiled for every platform (desktop, Android,
// iOS) since it's all plain Rust -- but the *implementation* of talking
// to a real native map view obviously can't be, since there's no
// `MapView` class outside Android. So the crate is split into two
// modules that both expose the same `GoogleMaps<R>` type/method names,
// and Rust's `#[cfg(...)]` picks exactly one of them to actually compile
// depending on the target -- `mobile.rs`'s real Android implementation,
// or `desktop.rs`'s stub (covering desktop *and* iOS, since neither has
// a real implementation here) that just returns `Error::Unsupported`
// from every method. Everything below this point calls into whichever
// one got compiled without needing to know which.
#[cfg(target_os = "android")]
mod mobile;
#[cfg(not(target_os = "android"))]
mod desktop;

mod commands;
mod error;
mod models;

// Re-exported so a consumer of this crate (the main app, via
// `tauri_plugin_google_maps::Error`) doesn't need to reach into
// `tauri_plugin_google_maps::error::Error` -- same reason most crates
// put their public types at the top level instead of leaving them
// nested in whatever module they happen to be implemented in.
pub use error::{Error, Result};
pub use models::*;

#[cfg(target_os = "android")]
use mobile::GoogleMaps;
#[cfg(not(target_os = "android"))]
use desktop::GoogleMaps;

// Tauri commands are free functions (see `commands.rs`), not methods on
// some shared object, so they need a way to reach the one `GoogleMaps`
// instance this plugin created at startup. `app.manage(value)` below is
// how any Tauri app stores that kind of shared, app-lifetime state --
// think of it as a type-keyed global map living on the `AppHandle`,
// retrieved later with `app.state::<GoogleMaps<R>>()`. This trait is
// just a thin wrapper around that lookup so call sites can write
// `app.google_maps()` instead of repeating the raw `state::<...>()` turbofish
// everywhere -- purely ergonomic, every official Tauri plugin does the
// same thing under a `<PluginName>Ext` name.
pub trait GoogleMapsExt<R: Runtime> {
    fn google_maps(&self) -> &GoogleMaps<R>;
}

impl<R: Runtime, T: Manager<R>> GoogleMapsExt<R> for T {
    fn google_maps(&self) -> &GoogleMaps<R> {
        self.state::<GoogleMaps<R>>().inner()
    }
}

// The function `src-tauri/src/lib.rs` actually calls
// (`.plugin(tauri_plugin_google_maps::init())`) -- everything else in
// this file exists to support what happens inside it.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("google-maps")
        // Registers the `#[command]` functions in `commands.rs` as the
        // set the frontend is allowed to call. Each is invoked from JS as
        // `invoke("plugin:google-maps|create_map", {...})` -- the
        // `plugin:<name>|` prefix (`"google-maps"`, the string passed to
        // `Builder::new` above) is what tells Tauri's IPC layer to route
        // the call here instead of to a plain app-level `#[tauri::command]`.
        .invoke_handler(tauri::generate_handler![
            commands::create_map,
            commands::update_frame,
            commands::set_camera,
            commands::set_marker,
            commands::dispose,
        ])
        // Runs exactly once, when the app starts and this plugin is
        // registered -- this is where `mobile::init`/`desktop::init`
        // (see those files) build the actual `GoogleMaps<R>` handle, which
        // then gets stashed via `app.manage(...)` (see the trait above)
        // so every later command invocation can find it again.
        .setup(|app, api| {
            #[cfg(target_os = "android")]
            let google_maps = mobile::init(app, api)?;
            #[cfg(not(target_os = "android"))]
            let google_maps = desktop::init(app, api)?;
            app.manage(google_maps);
            Ok(())
        })
        .build()
}
