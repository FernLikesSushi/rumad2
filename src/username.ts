// Persists `ConnectForm`'s username field across app restarts, same
// `localStorage` pattern as `devMode.ts`. Unlike devMode this isn't a
// shared reactive signal -- only `ConnectForm` itself reads/writes it --
// so a plain load/save pair is enough. Deliberately doesn't cover
// `password`: persisting a password in plaintext local storage isn't

import { createSignal } from "solid-js";

// worth the convenience here, so that field is always re-entered.
const STORAGE_KEY = "rumad-username";

export function loadUsername(): string {
  return typeof localStorage !== "undefined" ? (localStorage.getItem(STORAGE_KEY) ?? "") : "";
}

export function saveUsername(value: string) {
  if (typeof localStorage !== "undefined") {
    localStorage.setItem(STORAGE_KEY, value);
  }
}
// shared password signal (memory only)
export const [password, setPassword] = createSignal("");