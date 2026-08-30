// This is a *build* script (runs at `cargo build` time, before the crate
// itself compiles), separate from `src/lib.rs` -- its job is codegen and
// wiring, not anything that runs inside the app. `tauri_plugin::Builder`
// (from the `tauri-plugin` *build*-dependency in Cargo.toml, a different
// crate than the `tauri` runtime dependency) is what:
//   - generates the ACL permission scaffolding in `permissions/` for
//     each command listed below (so `default.toml`'s
//     `"allow-create-map"` etc. actually exist as real permissions),
//   - emits the `#[cfg(mobile)]`/`#[cfg(desktop)]` custom cfg flags
//     `lib.rs`/`error.rs` gate on (`mobile` = Android or iOS, `desktop` =
//     everything else) -- without this build script actually running,
//     those cfg names wouldn't exist and every `#[cfg(mobile)]`/
//     `#[cfg(desktop)]` in this crate would just silently evaluate to
//     false,
//   - and, via `.android_path("android")`/`.ios_path("ios")`, tells the
//     Tauri CLI where to find this plugin's native project for each
//     platform (the `android/` and `ios/` directories next to this
//     file) when it wires up `gen/android`'s `settings.gradle.kts` /
//     `gen/apple`'s Xcode project (via `project.yml`) to actually
//     include them as dependencies -- that wiring happens when `cargo
//     tauri android|ios init`/`dev`/`build` runs, not from this build
//     script itself.
const COMMANDS: &[&str] = &["create_map", "update_frame", "set_camera", "set_marker", "dispose"];

fn main() {
    tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        .ios_path("ios")
        .build();
}
