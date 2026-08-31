import { createSignal } from "solid-js";

const SHOW_MS = 3000;
// Matches `Toast.tsx`'s own fade-out transition duration -- a toast
// stays in `toasts` (just flagged `closing`) for this long after its
// `SHOW_MS` expires so that transition actually gets to play, instead
// of the item vanishing from the list (and the DOM) mid-fade.
export const EXIT_MS = 250;

export type ToastItem = { id: number; message: string; closing: boolean };

let nextId = 0;

// A queue, not a single overwritable slot -- each `showToast` call gets
// its own entry with its own lifetime, so a second toast while the
// first is still showing stacks alongside it (daisyUI's own `.toast`
// stacking pattern, `Toast.tsx` renders every entry inside one
// container) instead of cancelling/replacing it the way a single
// `[string | null]` signal did.
const [toasts, setToasts] = createSignal<ToastItem[]>([]);
export { toasts };

// Generic, feature-independent toast state -- any part of the app calls
// `showToast(message)`; `Toast.tsx` (mounted once, globally, in App.tsx)
// is the only thing that reads `toasts`.
export function showToast(message: string) {
  const id = nextId++;
  setToasts((list) => [...list, { id, message, closing: false }]);

  setTimeout(() => {
    // Flag the toast as closing so that `Toast.tsx` can apply its fade-out transition class, then remove it from the list after the transition duration.
    setToasts((list) => list.map((t) => (t.id === id ? { ...t, closing: true } : t)));
    
    setTimeout(() => {
      setToasts((list) => list.filter((t) => t.id !== id));
    }, EXIT_MS);
  }, SHOW_MS);
}
