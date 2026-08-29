import { createSignal, Show } from "solid-js";
import { t } from "../../i18n";

// `ClassPreview`'s "+" button opens this to name a new `ClassProfile`
// before it's saved -- same `modal`/`modal-box` overlay as `NoticeDialog`,
// but with a text field and Create/Cancel instead of a single dismiss.
export function NewProfileDialog(props: {
  open: boolean;
  existingNames: string[];
  onCreate: (name: string) => void;
  onClose: () => void;
}) {
  const [name, setName] = createSignal("");

  function isDuplicate() {
    return props.existingNames.includes(name().trim());
  }

  function cancel() {
    setName("");
    props.onClose();
  }

  function submit(e: Event) {
    e.preventDefault();
    const trimmed = name().trim();
    if (!trimmed || isDuplicate()) return;
    props.onCreate(trimmed);
    setName("");
  }

  return (
    <Show when={props.open}>
      <div class="modal modal-open" onClick={cancel}>
        <div class="modal-box" role="dialog" aria-modal="true" onClick={(e) => e.stopPropagation()}>
          <h3 class="font-bold text-lg">{t().newProfileDialog.title}</h3>
          <form onSubmit={submit}>
            <input
              class="input input-bordered w-full mt-4"
              type="text"
              placeholder={t().newProfileDialog.namePlaceholder}
              value={name()}
              onInput={(e) => setName(e.currentTarget.value)}
              autofocus
            />
            <Show when={name().trim() && isDuplicate()}>
              <p class="text-error text-sm mt-2">{t().newProfileDialog.duplicateError}</p>
            </Show>
            <div class="modal-action">
              <button type="button" class="btn" onClick={cancel}>
                {t().newProfileDialog.cancel}
              </button>
              <button type="submit" class="btn btn-primary" disabled={!name().trim() || isDuplicate()}>
                {t().newProfileDialog.create}
              </button>
            </div>
          </form>
        </div>
      </div>
    </Show>
  );
}
