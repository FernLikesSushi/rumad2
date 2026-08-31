import { For } from "solid-js";
import { toasts } from "../data/toast";

// Global, route-independent -- mounted once in App.tsx (same as
// `TuiDialogHandler`) so `showToast` works no matter which page called
// it. One `.toast` container, always mounted, stacking every queued
// entry inside it (daisyUI's own multi-toast pattern) -- each `<For>`
// item mounts fresh when added (replaying daisyUI's `.toast > *`
// entrance keyframe, `daisyui.css`) and stays mounted through its own
// `closing` flag's fade-out (`data/toast.ts` delays actually removing
// it from the list by that same duration) instead of popping away
// mid-transition.
export function Toast() {
  return (
    <div class="toast toast-top toast-end z-50">
      <For each={toasts()}>
        {(item) => (
          <div
            class="alert alert-success transition-opacity duration-250"
            classList={{ "opacity-0": item.closing }}
          >
            <span>{item.message}</span>
          </div>
        )}
      </For>
    </div>
  );
}
