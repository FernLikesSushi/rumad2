import { Show } from "solid-js";
import { currentToast } from "../data/toast";

// Global, route-independent -- mounted once in App.tsx (same as
// `TuiDialogHandler`) so `showToast` works no matter which page called it.
export function Toast() {
  return (
    <Show when={currentToast()}>
      {(message) => (
        <div class="toast toast-top toast-end z-50">
          <div class="alert alert-success">
            <span>{message()}</span>
          </div>
        </div>
      )}
    </Show>
  );
}
