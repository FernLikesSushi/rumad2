import { createResource } from "solid-js";
import { invoke } from "@tauri-apps/api/core";

// TODO: Implement connection persistence when switching routes.

async function checkConnected(): Promise<boolean> {
  try {
    return await invoke<boolean>("is_connected");
  } catch {
    return false;
  }
}

// Whether a TUI session is live, re-derived from the Rust backend (the
// real source of truth -- `AppState`) rather than tracked by hand on the
// frontend. `source` re-triggers the check whenever it changes; pass
// something like the current route so it's fresh after every navigation.
export function useConnected(source: () => unknown) {
  // const [connected] = createResource(source, checkConnected);
  // return connected;
  return () => false;
}
