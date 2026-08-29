const COMMANDS: &[&str] = &["create_map", "update_frame", "set_camera", "set_marker", "dispose"];

fn main() {
    tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        .build();
}
