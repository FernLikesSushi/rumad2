use tauri::{command, AppHandle, Runtime};

use crate::models::*;
use crate::GoogleMapsExt;
use crate::Result;

#[command]
pub(crate) async fn create_map<R: Runtime>(app: AppHandle<R>, payload: CreateMapArgs) -> Result<()> {
    app.google_maps().create_map(payload)
}

#[command]
pub(crate) async fn update_frame<R: Runtime>(app: AppHandle<R>, payload: UpdateFrameArgs) -> Result<()> {
    app.google_maps().update_frame(payload)
}

#[command]
pub(crate) async fn set_camera<R: Runtime>(app: AppHandle<R>, payload: SetCameraArgs) -> Result<()> {
    app.google_maps().set_camera(payload)
}

#[command]
pub(crate) async fn set_marker<R: Runtime>(app: AppHandle<R>, payload: SetMarkerArgs) -> Result<()> {
    app.google_maps().set_marker(payload)
}

#[command]
pub(crate) async fn dispose<R: Runtime>(app: AppHandle<R>) -> Result<()> {
    app.google_maps().dispose()
}
