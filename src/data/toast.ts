import { createMemo, createSignal } from "solid-js";

const TOAST_MS = 3000;

const [toast, setToast] = createSignal<string | null>(null);
let toastTimeout: ReturnType<typeof setTimeout>;

// Generic, feature-independent toast state -- any part of the app calls
// `showToast(message)`; `Toast.tsx` (mounted once, globally, in App.tsx)
// is the only thing that reads `currentToast`.
export const currentToast = createMemo(() => toast());

export function showToast(message: string) {
  setToast(message);
  clearTimeout(toastTimeout);
  toastTimeout = setTimeout(() => setToast(null), TOAST_MS);
}
