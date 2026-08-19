import { Show } from "solid-js";
import { t } from "../i18n";
import type { Dialog } from "../types";

export function NoticeDialog(props: { dialog: Dialog | null; onClose: () => void }) {
  return (
    <Show when={props.dialog}>
      {(d) => (
        <div class="dialog-overlay" onClick={props.onClose}>
          <div class="dialog" role="alertdialog" aria-modal="true" onClick={(e) => e.stopPropagation()}>
            <h3>{d().title}</h3>
            <p>{d().message}</p>
            <button onClick={props.onClose}>{t().close}</button>
          </div>
        </div>
      )}
    </Show>
  );
}
