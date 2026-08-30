// The desktop half of the `mobile`/`desktop` split described in
// `lib.rs` (Tauri's own cfg alias -- `#[cfg(desktop)]` there, so this
// covers Windows/macOS/Linux, not Android or iOS, both real now via
// `mobile.rs`). There's no native map view to talk to on desktop at
// all, so this exists purely so the crate *compiles* there and every
// command fails the same predictable way (`Error::Unsupported`) instead
// of the whole crate refusing to build on non-mobile targets.
use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;
use crate::Result;

// Mirrors `mobile::init`'s signature exactly (same function name, same
// params, same return type) -- that symmetry is what lets `lib.rs`'s
// `setup()` closure call `mobile::init(app, api)` or `desktop::init(app,
// api)` behind a `#[cfg(...)]` without the rest of the file needing to
// change. `api` (the mobile-plugin registration handle) goes unused here
// since there's no Kotlin/Swift class to register against.
pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<NativeMap<R>> {
    Ok(NativeMap(app.clone()))
}

/// No native map view on desktop (Windows/macOS/Linux have no such
/// concept) -- every method just reports unsupported so the frontend
/// falls back to `GoogleMapWeb`'s iframe embed there instead.
pub struct NativeMap<R: Runtime>(#[allow(dead_code)] AppHandle<R>);

// Every method below just returns the same error -- there's nothing
// platform-specific to *do* here, this only exists so calling
// `app.native_map().create_map(...)` (see `commands.rs`) type-checks
// and behaves predictably regardless of which module (`mobile`/
// `desktop`) actually got compiled in.
impl<R: Runtime> NativeMap<R> {
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
