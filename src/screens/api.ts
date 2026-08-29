import { invoke } from "@tauri-apps/api/core";
import type { Action, ClassifiedScreen, Send, TuiScreen } from "../data/tui";

// The resource's fetcher: `null` means "session ended",
// anything else is whatever the invoked command
// resolved to.
export async function runAction(action: Action): Promise<ClassifiedScreen | null> {
  if (action.cmd === "disconnect") {
    await invoke("disconnect");
    return null;
  }
  return invoke<ClassifiedScreen>(action.cmd, action.args);
}

// Shared prop shape for every screen component that renders a specific
// `TuiScreen` variant (everything `TuiSession.tsx`'s `Switch`/`Match`
// dispatches to except `LoginScreen`/`DisconnectedScreen`, whose kinds
// carry no data) -- `screen` is narrowed to just that variant via `T`, so
// the component destructures its own fields off `props.screen` instead of
// the caller unpacking them into separate named props.
export type TuiScreenComponentProps<T extends TuiScreen["kind"]> = {
  screen: Extract<TuiScreen, { kind: T }>;
  busy: boolean;
  send: Send;
};
