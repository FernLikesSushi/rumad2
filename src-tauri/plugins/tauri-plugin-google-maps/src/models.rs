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
