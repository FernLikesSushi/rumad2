//! App-internal Tauri plugin embedding a real native map view alongside
//! the webview -- Android's `com.google.android.gms.maps.MapView`
//! (Google Maps SDK, needs `VITE_GOOGLE_MAPS_API_KEY`), iOS's
//! `MKMapView` (Apple's own MapKit -- no API key at all, it's a system
//! framework). A plain `<iframe>` (the `GoogleMapWeb` component) has no
//! native-map affordances (gestures, markers, my-location) and looks
//! noticeably worse than a real map surface on a phone. Desktop has no
//! native view here -- `desktop.rs` covers it, returning `Error::
//! Unsupported` for every command, and the frontend falls back to
//! `GoogleMapWeb` there instead (see `GoogleMapNative.tsx`).
//!
//! Named `native-map`, not `google-maps` -- despite the crate directory
//! having started life Android-only (Google Maps SDK), it's a genuinely
//! platform-agnostic "give me a real native map here" plugin now that
//! iOS uses Apple's own MapKit, not Google's SDK at all. `GoogleMapWeb`/
//! `GoogleMapNative.tsx` on the frontend keep their names regardless
//! (the *web* fallback really is always Google Maps' iframe embed).
//!
//! Neither native view is part of the DOM -- each is a separate native
//! view layered on top of the WebView's own view, positioned to match a
//! placeholder element's `getBoundingClientRect()` (see `Frame` in
//! `models.rs`). `create_map`/`update_frame` reposition it; `set_camera`/
//! `set_marker` change what it's showing without moving it; `dispose`
//! tears it down (called on the frontend component's `onCleanup`). The
//! command surface (`commands.rs`) and payload shapes (`models.rs`) are
//! shared between Android and iOS -- only `mobile.rs`'s two native
//! implementations differ.

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
// to a real native map view obviously can't be, since desktop has no
// such concept at all. So the crate is split into two modules that both
// expose the same `NativeMap<R>` type/method names, and Rust's
// `#[cfg(...)]` picks exactly one of them to actually compile depending
// on the target -- `mobile.rs` (real Android + iOS implementations, one
// `#[cfg(target_os = ...)]`-gated branch each inside that single file --
// see there), or `desktop.rs`'s stub that just returns `Error::
// Unsupported` from every method. `mobile`/`desktop` here are Tauri's
// own built-in cfg aliases (set up by the `tauri-plugin` build
// dependency's `Builder::build()` in `build.rs`), not something this
// crate defines -- `mobile` is true for Android *or* iOS, `desktop` is
// every other target. Everything below this point calls into whichever
// one got compiled without needing to know which.
#[cfg(mobile)]
mod mobile;
#[cfg(desktop)]
mod desktop;

mod commands;
mod error;
mod models;

// Re-exported so a consumer of this crate (the main app, via
// `tauri_plugin_native_map::Error`) doesn't need to reach into
// `tauri_plugin_native_map::error::Error` -- same reason most crates
// put their public types at the top level instead of leaving them
// nested in whatever module they happen to be implemented in.
pub use error::{Error, Result};
pub use models::*;

#[cfg(mobile)]
use mobile::NativeMap;
#[cfg(desktop)]
use desktop::NativeMap;

// Tauri commands are free functions (see `commands.rs`), not methods on
// some shared object, so they need a way to reach the one `NativeMap`
// instance this plugin created at startup. `app.manage(value)` below is
// how any Tauri app stores that kind of shared, app-lifetime state --
// think of it as a type-keyed global map living on the `AppHandle`,
// retrieved later with `app.state::<NativeMap<R>>()`. This trait is
// just a thin wrapper around that lookup so call sites can write
// `app.native_map()` instead of repeating the raw `state::<...>()` turbofish
// everywhere -- purely ergonomic, every official Tauri plugin does the
// same thing under a `<PluginName>Ext` name.
pub trait NativeMapExt<R: Runtime> {
    fn native_map(&self) -> &NativeMap<R>;
}

impl<R: Runtime, T: Manager<R>> NativeMapExt<R> for T {
    fn native_map(&self) -> &NativeMap<R> {
        self.state::<NativeMap<R>>().inner()
    }
}

// The function `src-tauri/src/lib.rs` actually calls
// (`.plugin(tauri_plugin_native_map::init())`) -- everything else in
// this file exists to support what happens inside it.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("native-map")
        // Registers the `#[command]` functions in `commands.rs` as the
        // set the frontend is allowed to call. Each is invoked from JS as
        // `invoke("plugin:native-map|create_map", {...})` -- the
        // `plugin:<name>|` prefix (`"native-map"`, the string passed to
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
        // (see those files) build the actual `NativeMap<R>` handle, which
        // then gets stashed via `app.manage(...)` (see the trait above)
        // so every later command invocation can find it again.
        .setup(|app, api| {
            #[cfg(mobile)]
            let native_map = mobile::init(app, api)?;
            #[cfg(desktop)]
            let native_map = desktop::init(app, api)?;
            app.manage(native_map);
            Ok(())
        })
        .build()
}
