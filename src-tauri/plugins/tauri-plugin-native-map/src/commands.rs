// This is the layer the frontend actually talks to. `invoke("plugin:
// native-map|create_map", { payload })` from `GoogleMapNative.tsx`
// resolves to a call of the `create_map` function below (the mapping is
// name-based -- "create_map" here has to match what's passed in
// `lib.rs`'s `invoke_handler!` list, which it does since they're the
// same `commands::create_map` item).
//
// Every command here is intentionally a one-line pass-through: get the
// shared `NativeMap<R>` state (`app.native_map()`, from the extension
// trait in `lib.rs`) and call the same-named method on it. The actual
// logic lives one level down, in `mobile.rs`/`desktop.rs` -- this file
// exists only because Tauri commands specifically have to be free
// functions taking `AppHandle`/args and marked `#[command]`, they can't
// just be methods on `NativeMap<R>` directly.
use tauri::{command, AppHandle, Runtime};

use crate::models::*;
use crate::NativeMapExt;
use crate::Result;

// `async fn` on all five, even though nothing here actually awaits
// anything -- this is about which *thread* the command runs on, not
// about awaiting: a plain (non-`async fn`) command runs inline on
// whatever thread dispatched the IPC message (the UI/main-loop thread on
// desktop), so a slow one would freeze the window. `async fn` instead
// gets dispatched onto Tauri's async runtime, off that thread -- same
// reasoning as this app's own `commands/*.rs` (see their doc comments).
// None of these five are slow enough for it to matter today, but it's
// the safe default for anything that talks to a native SDK.
//
// All five are called from `GoogleMapNative.tsx`, which is the only
// caller -- see each one's own comment for exactly when.

/// Creates the native map view (Android's `MapView`, iOS's `MKMapView`),
/// adds it to the app's view tree at the placeholder element's screen
/// position, and sets the starting camera position and drops a marker.
/// Called once, in `GoogleMapNative.tsx`'s `onMount`.
#[command]
pub(crate) async fn create_map<R: Runtime>(app: AppHandle<R>, payload: CreateMapArgs) -> Result<()> {
    app.native_map().create_map(payload)
}

/// Repositions/resizes the already-created native map view to match the
/// placeholder's current `getBoundingClientRect()` -- no camera/marker
/// change. Called on every `ResizeObserver` firing, window `resize`, and
/// window `scroll`; a no-op on the JS side until `create_map` has
/// actually resolved (there's no map view to reposition before then).
#[command]
pub(crate) async fn update_frame<R: Runtime>(app: AppHandle<R>, payload: UpdateFrameArgs) -> Result<()> {
    app.native_map().update_frame(payload)
}

/// Moves the existing map's camera to a new lat/lng/zoom. Called
/// whenever `GoogleMapNative`'s `lat`/`lng`/`zoom` props change *after*
/// the initial mount -- e.g. navigating from one room's `Map` route to
/// another without unmounting the component -- always alongside
/// `set_marker` (see below), never on its own.
#[command]
pub(crate) async fn set_camera<R: Runtime>(app: AppHandle<R>, payload: SetCameraArgs) -> Result<()> {
    app.native_map().set_camera(payload)
}

/// Removes the old marker (if any) and adds a new one at a new
/// position/title, without moving the camera itself. Same trigger as
/// `set_camera` -- the two are always called together in
/// `GoogleMapNative.tsx`'s prop-change effect.
#[command]
pub(crate) async fn set_marker<R: Runtime>(app: AppHandle<R>, payload: SetMarkerArgs) -> Result<()> {
    app.native_map().set_marker(payload)
}

/// Removes the native map view from the view tree and tears down its
/// resources. Called once, in `GoogleMapNative.tsx`'s `onCleanup` --
/// i.e. when the component unmounts (navigating away from `/map`).
#[command]
pub(crate) async fn dispose<R: Runtime>(app: AppHandle<R>) -> Result<()> {
    app.native_map().dispose()
}
