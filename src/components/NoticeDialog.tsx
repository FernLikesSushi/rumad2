import { createMemo, createSignal } from "solid-js";
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
  // Kept mounted at all times, `.modal-open` toggled instead of the
  // element being added/removed (the old `<Show>`) -- daisyUI's own CSS
  // already animates `.modal`/`.modal-box` open *and* closed (fade +
  // scale, see `daisyui.css`'s `.modal`/`.modal.modal-open` rules), but
  // only if the element survives long enough for the closing transition
  // to actually play. `<Show>` unmounted it the instant `dialog` went
  // null, before that transition had a chance to run at all.
  //
  // `shown` retains the last non-null dialog while `props.dialog` is
  // briefly null again (closing) -- same previous-value-reducer pattern
  // `App.tsx` already uses for its own Notice memo, so the modal fades
  // out still showing its last content instead of popping to blank.
  const shown = createMemo<DialogBox | null>((prev) => props.dialog ?? prev ?? null);

  return (
    <div class="modal" classList={{ "modal-open": props.dialog !== null }} onClick={props.onClose}>
      <div
        class="modal-box"
        role="alertdialog"
        aria-modal="true"
        onClick={(e) => e.stopPropagation()}
      >
        <h3 class="font-bold text-lg">{shown()?.title}</h3>
        <p class="py-4">{shown()?.message}</p>
        <div class="modal-action">
          <button class="btn" onClick={props.onClose}>
            {t().close}
          </button>
        </div>
      </div>
    </div>
  );
}
