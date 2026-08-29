// The Android half of the `mobile.rs`/`desktop.rs` split described in
// `lib.rs`. This file's job is narrow: get a handle to the real Kotlin
// plugin class (`GoogleMapsPlugin.kt`, in `android/src/main/java/...`)
// and expose Rust methods that call into it. None of the actual map
// logic lives here -- it's all on the Kotlin side.
use serde::de::DeserializeOwned;
use tauri::{
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};

use crate::models::*;
use crate::Result;

// Called once from `lib.rs`'s `init()` (via `mobile::init(app, api)`)
// when the app starts. `PluginApi` is Tauri's handle to "the mobile side
// of this specific plugin" -- `register_android_plugin` is what actually
// looks up the Kotlin class by its fully-qualified name + class name
// (matching the `package`/`class GoogleMapsPlugin` declared in the .kt
// file) and wires up the IPC bridge to it, returning a `PluginHandle`
// that can send it commands.
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
    // `run_mobile_plugin(commandName, payload)` is the actual RPC call:
    // it serializes `payload` to JSON, sends it across to the Kotlin
    // plugin instance held by `self.0`, and blocks until that side calls
    // `invoke.resolve(...)`/`invoke.reject(...)` (see `@Command fun
    // createMap(invoke: Invoke)` in the .kt file) -- the string here
    // ("createMap") has to match that Kotlin method's name exactly,
    // that's the only thing connecting the two sides. There's no
    // compile-time check tying "createMap" to the Kotlin method, unlike
    // the Rust-side command names in `commands.rs`, which the compiler
    // does check against `invoke_handler!`.
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

    // `()` as the payload -- `dispose` takes no arguments on either side
    // (`Invoke` on the Kotlin side just won't have any args to parse).
    pub fn dispose(&self) -> Result<()> {
        self.0.run_mobile_plugin("dispose", ()).map_err(Into::into)
    }
}
