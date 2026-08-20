import { Show } from "solid-js";
import { t } from "../i18n";
import type { DialogBox } from "../types";
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
