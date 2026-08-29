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

use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

#[cfg(target_os = "android")]
mod mobile;
#[cfg(not(target_os = "android"))]
mod desktop;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};
pub use models::*;

#[cfg(target_os = "android")]
use mobile::GoogleMaps;
#[cfg(not(target_os = "android"))]
use desktop::GoogleMaps;

/// Extension trait mirroring every other Tauri plugin's own `*Ext`
/// convention -- `app.google_maps()` instead of threading a `State`
/// through each command by hand.
pub trait GoogleMapsExt<R: Runtime> {
    fn google_maps(&self) -> &GoogleMaps<R>;
}

impl<R: Runtime, T: Manager<R>> GoogleMapsExt<R> for T {
    fn google_maps(&self) -> &GoogleMaps<R> {
        self.state::<GoogleMaps<R>>().inner()
    }
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("google-maps")
        .invoke_handler(tauri::generate_handler![
            commands::create_map,
            commands::update_frame,
            commands::set_camera,
            commands::set_marker,
            commands::dispose,
        ])
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
