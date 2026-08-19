import { createSignal } from "solid-js";

const STORAGE_KEY = "rumad-dev-mode";

function loadInitial(): boolean {
  return typeof localStorage !== "undefined" && localStorage.getItem(STORAGE_KEY) === "true";
}

const [devMode, setDevModeSignal] = createSignal<boolean>(loadInitial());
export { devMode };

export function setDevMode(value: boolean) {
  setDevModeSignal(value);
  if (typeof localStorage !== "undefined") {
    localStorage.setItem(STORAGE_KEY, String(value));
  }
}
