import { invoke } from "@tauri-apps/api/core";

// A light tap -- meant to be called only from the handful of places
// that actually do something meaningful (e.g. `data/tui.ts`'s
// `send`/`connect`/`login`/`disconnect`), not a blanket listener on
// every `.btn` (nav/settings/close chrome included), the way a real
// iOS app reserves haptics for meaningful actions rather than every
// tap. `haptic_light` (`commands/mod.rs`) is a no-op on every platform
// but iOS.
export function haptic() {
  void invoke("haptic_light");
}
