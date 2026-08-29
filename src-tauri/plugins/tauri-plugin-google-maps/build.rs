// This is a *build* script (runs at `cargo build` time, before the crate
// itself compiles), separate from `src/lib.rs` -- its job is codegen and
// wiring, not anything that runs inside the app. `tauri_plugin::Builder`
// (from the `tauri-plugin` *build*-dependency in Cargo.toml, a different
// crate than the `tauri` runtime dependency) is what:
//   - generates the ACL permission scaffolding in `permissions/` for
//     each command listed below (so `default.toml`'s
//     `"allow-create-map"` etc. actually exist as real permissions),
//   - emits the `#[cfg(mobile)]`/`#[cfg(desktop)]` custom cfg flags other
//     Tauri plugins commonly gate on (this crate uses its own explicit
//     `#[cfg(target_os = "android")]` instead -- see `lib.rs` -- since
//     only Android has a real implementation, but the flags still get
//     emitted either way),
//   - and, via `.android_path("android")`, tells the Tauri CLI where to
//     find this plugin's Android Gradle module (the `android/` directory
//     next to this file) when it wires up `gen/android`'s
//     `settings.gradle.kts` -- that wiring happens when `cargo tauri
//     android init`/`dev`/`build` runs, not from this build script
//     itself.
const COMMANDS: &[&str] = &["create_map", "update_frame", "set_camera", "set_marker", "dispose"];

fn main() {
    tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        .build();
}
