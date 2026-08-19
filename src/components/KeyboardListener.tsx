import { onCleanup } from "solid-js";

// A plain primitive, not a wrapping component -- this renders nothing, so
// there's no reason to wrap `children` in JSX just to run a side effect
// (unlike React, Solid doesn't need that). Call directly from a
// component's body; the listener attaches immediately and detaches via
// `onCleanup` when that component unmounts.
export function createKeyboardListener(onKeyDown: (key: string) => void) {
  const handleKeyDown = (event: KeyboardEvent) => onKeyDown(event.key);
  
  window.addEventListener("keydown", handleKeyDown);
  onCleanup(() => window.removeEventListener("keydown", handleKeyDown));
}
