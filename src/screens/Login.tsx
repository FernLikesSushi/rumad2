import { For } from "solid-js";
import { t } from "../i18n";
import type { LoginField } from "../types";

export function LoginScreen(props: {
  fields: LoginField[];
  values: Record<string, string>;
  onChange: (key: string, value: string) => void;
  onSubmit: (e: Event) => void;
  busy: boolean;
}) {
  return (
    <>
      <h2>{t().authTitle}</h2>
      <form class="login" onSubmit={props.onSubmit}>
        <For each={props.fields}>
          {(field) => (
            <input
              type="password"
              placeholder={`${field.label} (${field.hint})`}
              value={props.values[field.key] ?? ""}
              onInput={(e) => props.onChange(field.key, e.currentTarget.value)}
            />
          )}
        </For>
        <button type="submit" disabled={props.busy}>
          {t().send}
        </button>
      </form>
    </>
  );
}
