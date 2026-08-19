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
  if (action.cmd === "login") {
    // Fixed-width auto-advancing fields, confirmed by the user: fill each
    // one in order with no separator keystroke -- the remote warns
    // against pressing Enter, so `send_text` (no trailing Enter) is used
    // for every field, including the last.
    const { idNumber, accessCode, ssnLast4, birthDate } = action.args;
    await invoke<TuiScreen>("send_text", { text: idNumber });
    await invoke<TuiScreen>("send_text", { text: accessCode });
    await invoke<TuiScreen>("send_text", { text: ssnLast4 });
    return invoke<TuiScreen>("send_text", { text: birthDate });
  }
  return invoke<TuiScreen>(action.cmd, action.args);
}
