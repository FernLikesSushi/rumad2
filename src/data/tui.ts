import { createMemo, createResource, createRoot, createSignal } from "solid-js";
import { listen } from "@tauri-apps/api/event";
import { runAction } from "../screens/api";
import type { Action, ClassifiedScreen, Send, TuiScreen } from "../types";

// Module-level (not owned by any component) so the current screen/busy
// state survives navigating away from "/session" and back (e.g. to
// "/map") instead of being torn down and refetched -- `TuiSession.tsx`
// renders whatever's here rather than owning it itself. The real source of
// truth is still the Rust backend's `AppState`; this is just a persistent
// cache of its last response, kept in sync the same way it always was: the
// `action`/`response` resource, plus the `screen-changed` event for
// redraws the backend pushes on its own. Dialog-showing is a separate
// concern -- `TuiDialogHandler` reacts to `response` below rather than
// this module owning a dialog signal itself.

// `false`, not an initial `get_screen` action -- this module is imported
// (transitively, via `Drawer.tsx`) before anyone has connected at all, and
// `false` is `createResource`'s built-in way to say "don't fetch yet".
// `ensureStarted` kicks off the real first fetch once `TuiSession.tsx`
// actually mounts (post-connect); a `false` source doubles as "no live
// session" for `connected` below.
const [action, setAction] = createSignal<Action | false>(false);

export function ensureStarted() {
  if (action() === false) setAction({ cmd: "get_screen", args: {} });
}

export function connect(username?: string, password?: string) {
  setAction({ cmd: "connect", args: { username: username || undefined, password: password || undefined } });
}

// Only the resource/memos need an owning root -- Solid's documented
// pattern for a permanent, app-lifetime store (as opposed to one scoped
// to a component), so it doesn't warn "computations created outside a
// createRoot... will never be disposed": never disposing is the point
// here, not an oversight.
export const { response, screen, busy, canExit, canContinue, connected, mutate } = createRoot(() => {
  const [response, { mutate }] = createResource(action, runAction);

  // `Dialog::Processing` counts as busy too. Must check response.error
  // before response() -- a resolved rejection otherwise throws uncaught
  // inside this memo.
  const busy = createMemo(() => {
    if (response.loading) return true;
    if (response.error) return false;
    return response()?.dialog?.kind === "Processing";
  });

  // Same response.error-before-response() rule as `busy` -- a rejected
  // command (e.g. a restricted MainMenu option) throws here otherwise.
  const screen = createMemo<TuiScreen | undefined>((prev) => {
    if (response.loading || response.error) return prev;
    return response()?.screen ?? prev;
  });

  const canExit = createMemo<boolean>((prev) => {
    if (response.loading || response.error) return prev;
    return response()?.canExit ?? prev;
  }, false);

  const canContinue = createMemo<boolean>((prev) => {
    if (response.loading || response.error) return prev;
    return response()?.canContinue ?? prev;
  }, false);

  // Whether a TUI session is live -- derived straight from `screen`, not a
  // separate `is_connected` poll: `screen` is already kept in sync with
  // the real backend state via `screen-changed` (Rust pushes that the
  // moment it confirms the SSH channel actually closed, see
  // `handle_scene_change`/`Disconnected` in commands/exec.rs), so re-asking
  // over IPC would just be checking the same truth a second, slower way.
  const connected = createMemo(() => screen() !== undefined && screen()?.kind !== "Disconnected");

  return { response, screen, busy, canExit, canContinue, connected, mutate };
});

// Backend polls in the background for screens that redraw a second time on
// their own (`Dialog::Processing`) and pushes this event when done, and
// also emits it directly from the `disconnect` command -- one event either
// way a session ends, so this is the single place that reacts to that.
// Attached once, for the app's lifetime, rather than per-mount -- there's
// no route-scoped onCleanup to unlisten from anymore.
listen<ClassifiedScreen>("screen-changed", (event) => {
  mutate(event.payload);
  // Reset back to "no live session" -- otherwise `action` would keep
  // pointing at whatever was last dispatched against the now-dropped
  // session, and `ensureStarted`'s `action() === false` check would never
  // see a reason to fetch again the next time "/session" mounts.
  if (event.payload.screen.kind === "Disconnected") setAction(false);
});

// quick helper to avoid repeating the `setAction({cmd:"send",args:{action}})` boilerplate everywhere
export const send: Send = (sendAction) => setAction({ cmd: "send", args: { action: sendAction } });

export function exitScreen() {
  send({ kind: "Exit" });
}

export function continueScreen() {
  send({ kind: "Continue" });
}

export function login(idNumber: string, accessCode: string, ssnLast4: string, birthDate: string) {
  setAction({ cmd: "login", args: { idNumber, accessCode, ssnLast4, birthDate } });
}

// Just the backend call -- navigating back to "/" afterward is the
// caller's job (`TuiSession.tsx` has the router context this doesn't).
export async function disconnect() {
  await runAction({ cmd: "disconnect", args: {} });
}
