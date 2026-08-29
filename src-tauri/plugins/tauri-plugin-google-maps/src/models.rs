// The argument types for each command in `commands.rs`. These have to
// exist on *both* sides of the JS<->Rust<->Kotlin chain, and nothing
// checks that they actually match except careful naming: the JSON shape
// `serde::Deserialize` expects here has to line up with what
// `GoogleMapNative.tsx`'s `invoke()` calls send, *and* with the
// `@InvokeArg`-annotated Kotlin classes in `GoogleMapsPlugin.kt` (since
// `run_mobile_plugin` in `mobile.rs` just forwards the same struct on as
// JSON again). `#[serde(rename_all = "camelCase")]` is what would make a
// multi-word field line up (Rust convention is snake_case, JS/Kotlin is
// camelCase) -- none of the fields below actually need it since they're
// all single words, but it's kept on every struct here as the safe
// default so adding a multi-word field later doesn't silently break.
use serde::Deserialize;

/// The placeholder element's `getBoundingClientRect()` in CSS px, as
/// measured by the frontend -- converted to real device px on the Kotlin
/// side (multiplied by the display density) before positioning the
/// native `MapView`, since Android `View.layout()` takes raw pixels.
#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Frame {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

// `#[serde(flatten)]` below means `CreateMapArgs`'s JSON has `x`/`y`/
// `width`/`height` sitting directly alongside `lat`/`lng`/`zoom` at the
// top level -- not nested under a `"frame": {...}` key the way the
// struct's own Rust shape might suggest. This has to match whatever
// shape the frontend actually sends (see `GoogleMapNative.tsx`'s
// `{ ...frame(), lat, lng, zoom }` spread) and what the Kotlin
// `CreateMapArgs` class expects (which just lists all seven fields flat,
// since Kotlin's arg parser has no equivalent "flatten" concept).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateMapArgs {
    #[serde(flatten)]
    pub frame: Frame,
    pub lat: f64,
    pub lng: f64,
    pub zoom: f32,
}

/// Reposition/resize the already-created `MapView` -- called on scroll or
/// layout changes, without touching the camera or marker.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateFrameArgs {
    #[serde(flatten)]
    pub frame: Frame,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetCameraArgs {
    pub lat: f64,
    pub lng: f64,
    pub zoom: f32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetMarkerArgs {
    pub lat: f64,
    pub lng: f64,
    pub title: Option<String>,
}
