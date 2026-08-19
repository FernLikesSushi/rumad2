import { invoke } from "@tauri-apps/api/core";
import type { Action, TuiScreen } from "./types";

// The resource's fetcher: `null` means "session ended",
// anything else is whatever the invoked command
// resolved to.
export async function runAction(action: Action): Promise<TuiScreen | null> {
  if (action.cmd === "disconnect") {
    await invoke("disconnect");
    return null;
  }
  return invoke<TuiScreen>(action.cmd, action.args);
}
