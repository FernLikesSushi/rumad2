import { platform } from "@tauri-apps/plugin-os";

// Tauri's own recommended way to check the current OS (docs: "Platform" /
// `@tauri-apps/plugin-os`) rather than sniffing `navigator.userAgent` --
// `platform()` is synchronous once the plugin's initialized, reading the
// value the Rust side (`tauri_plugin_os::init()`, registered in
// `src-tauri/src/lib.rs`) resolved at startup.
export function isAndroid(): boolean {
  return platform() === "android";
}

export function isMobile(): boolean {
  const p = platform();
  return p === "android" || p === "ios";
}
