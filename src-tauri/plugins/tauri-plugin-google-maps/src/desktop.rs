use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;
use crate::Result;

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<GoogleMaps<R>> {
    Ok(GoogleMaps(app.clone()))
}

/// No native `MapView` outside Android (desktop has no such concept, and
/// iOS isn't implemented yet) -- every method just reports unsupported so
/// the frontend falls back to `GoogleMapWeb`'s iframe embed there instead.
pub struct GoogleMaps<R: Runtime>(#[allow(dead_code)] AppHandle<R>);

impl<R: Runtime> GoogleMaps<R> {
    pub fn create_map(&self, _payload: CreateMapArgs) -> Result<()> {
        Err(crate::Error::Unsupported)
    }

    pub fn update_frame(&self, _payload: UpdateFrameArgs) -> Result<()> {
        Err(crate::Error::Unsupported)
    }

    pub fn set_camera(&self, _payload: SetCameraArgs) -> Result<()> {
        Err(crate::Error::Unsupported)
    }

    pub fn set_marker(&self, _payload: SetMarkerArgs) -> Result<()> {
        Err(crate::Error::Unsupported)
    }

    pub fn dispose(&self) -> Result<()> {
        Err(crate::Error::Unsupported)
    }
}
