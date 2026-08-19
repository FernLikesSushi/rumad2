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
    <form class="row" onSubmit={handleSubmit}>
      <input
        placeholder={t().sendPlaceholder}
        value={props.value}
        onInput={(e) => props.onInput(e.currentTarget.value)}
      />
      <button type="submit" disabled={props.busy}>
        {t().send}
      </button>
    </form>
  );
}
