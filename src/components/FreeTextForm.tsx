import { t } from "../i18n";

export function FreeTextForm(props: {
  value: string;
  onInput: (value: string) => void;
  onSubmit: (e: Event) => void;
  busy: boolean;
}) {
  return (
    <form class="row" onSubmit={props.onSubmit}>
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
