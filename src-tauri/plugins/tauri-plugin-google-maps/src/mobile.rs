use serde::de::DeserializeOwned;
use tauri::{
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};

use crate::models::*;
use crate::Result;

pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> crate::Result<GoogleMaps<R>> {
    let handle = api.register_android_plugin("me.fern.rumad2.googlemaps", "GoogleMapsPlugin")?;
    Ok(GoogleMaps(handle))
}

/// Handle to the Kotlin `GoogleMapsPlugin` -- each method is a thin
/// wrapper over `run_mobile_plugin`, mirroring the `@Command`-annotated
/// methods on the Android side one-for-one.
pub struct GoogleMaps<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> GoogleMaps<R> {
    pub fn create_map(&self, payload: CreateMapArgs) -> Result<()> {
        self.0.run_mobile_plugin("createMap", payload).map_err(Into::into)
    }

    pub fn update_frame(&self, payload: UpdateFrameArgs) -> Result<()> {
        self.0.run_mobile_plugin("updateFrame", payload).map_err(Into::into)
    }

    pub fn set_camera(&self, payload: SetCameraArgs) -> Result<()> {
        self.0.run_mobile_plugin("setCamera", payload).map_err(Into::into)
    }

    pub fn set_marker(&self, payload: SetMarkerArgs) -> Result<()> {
        self.0.run_mobile_plugin("setMarker", payload).map_err(Into::into)
    }

    pub fn dispose(&self) -> Result<()> {
        self.0.run_mobile_plugin("dispose", ()).map_err(Into::into)
    }
}
