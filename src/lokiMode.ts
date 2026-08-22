import { createSignal } from "solid-js";

const STORAGE_KEY = "rumad-loki-mode";

function loadInitial(): boolean {
  return typeof localStorage !== "undefined" && localStorage.getItem(STORAGE_KEY) === "true";
}

const [lokiMode, setLokiModeSignal] = createSignal<boolean>(loadInitial());
export { lokiMode };

export function setLokiMode(value: boolean) {
  setLokiModeSignal(value);
  if (typeof localStorage !== "undefined") {
    localStorage.setItem(STORAGE_KEY, String(value));
  }
}
