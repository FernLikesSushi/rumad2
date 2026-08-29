import { createSignal, Show } from "solid-js";
import { t } from "../i18n";

// UI state for the notice dialog below -- derived from a `Dialog`
// (specifically `Notice`; `Processing` doesn't open this), not the same
// thing as one.
type DialogBox = { title: string; message: string };

// Owns the show/dismiss state a `<NoticeDialog>` needs -- `TuiRouter` and
// `Connect` each open one the same way (an error dialog on a rejected
// action, `TuiRouter` additionally for a `Dialog::Notice`), so this pairs
// with the component below rather than each caller hand-rolling its own
// signal.
export function createDialog() {
  const [dialog, setDialog] = createSignal<DialogBox | null>(null);
  return {
    dialog,
    show: (box: DialogBox) => setDialog(box),
    close: () => setDialog(null),
  };
}

export function NoticeDialog(props: { dialog: DialogBox | null; onClose: () => void }) {
  return (
    <Show when={props.dialog}>
      {(d) => (
        <div class="modal modal-open" onClick={props.onClose}>
          <div
            class="modal-box"
            role="alertdialog"
            aria-modal="true"
            onClick={(e) => e.stopPropagation()}
          >
            <h3 class="font-bold text-lg">{d().title}</h3>
            <p class="py-4">{d().message}</p>
            <div class="modal-action">
              <button class="btn" onClick={props.onClose}>
                {t().close}
              </button>
            </div>
          </div>
        </div>
      )}
    </Show>
  );
}
