import { For } from "solid-js";
import { t } from "../i18n";
import type { LoginField } from "../types";

// Only the access code (PIN) and SSN digits are sensitive enough to mask
// on screen -- the ID number and birth date aren't secret on their own.
const MASKED_FIELDS = new Set(["access_code", "ssn_last4"]);

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
              type={MASKED_FIELDS.has(field.key) ? "password" : "text"}
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
