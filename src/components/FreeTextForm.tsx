import { t } from "../i18n";

export function FreeTextForm(props: {
  value: string;
  onInput: (value: string) => void;
  onSubmit: (text: string) => void;
  busy: boolean;
}) {
  function handleSubmit(e: Event) {
    e.preventDefault();
    props.onSubmit(props.value);
  }

  return (
    <form class="flex justify-center" onSubmit={handleSubmit}>
      <input
        class="input"
        placeholder={t().sendPlaceholder}
        value={props.value}
        onInput={(e) => props.onInput(e.currentTarget.value)}
      />
      <button type="submit" class="btn btn-outline btn-primary" disabled={props.busy}>
        {t().send}
      </button>
    </form>
  );
}
