// A plugin's error type has to implement `serde::Serialize`, not just
// `std::error::Error` -- when a command returns `Err(...)`, Tauri sends
// it back across IPC as JSON so the frontend's `await invoke(...)` can
// reject with it, and only `Serialize` types can cross that boundary.
// `thiserror::Error` (the derive below) gives every variant a `Display`
// impl (the `#[error("...")]` messages) but says nothing about JSON --
// that's what the hand-written `impl Serialize` at the bottom is for: it
// just serializes the error as its `Display` string.
use serde::{Serialize, Serializer};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    // Only exists on mobile (Android or iOS) -- `PluginInvokeError` is
    // what `run_mobile_plugin` (see `mobile.rs`) can fail with, e.g. if
    // the Kotlin/Swift side's `invoke.reject(...)` gets called, or the
    // IPC message itself couldn't be delivered. `#[from]` is what lets
    // `mobile.rs` write `.map_err(Into::into)` instead of matching on
    // the error by hand.
    #[cfg(mobile)]
    #[error(transparent)]
    PluginInvoke(#[from] tauri::plugin::mobile::PluginInvokeError),
    /// Desktop only -- `desktop.rs` returns this for every command;
    /// callers should fall back to a web embed instead of treating it as
    /// a hard failure.
    #[error("the native map view is only implemented on Android/iOS")]
    Unsupported,
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
