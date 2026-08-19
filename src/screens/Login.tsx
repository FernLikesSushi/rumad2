import { createSignal, For } from "solid-js";
import { t } from "../i18n";

// Hardcoded rather than read from the backend's `fields` -- the remote's
// own labels for this specific form arrive mangled over the wire (see
// `LoginField`'s doc comment on the Rust side, which hardcodes the same
// text for the same reason), and the field set/order is fixed regardless.
// Unlike other remote-mirroring text, this is safe to localize since it's
// a fixed, known set rather than open-ended scraped content -- see
// `Messages.loginFields`'s doc comment.
function fields() {
  const messages = t().loginFields;
  return [
    { key: "id_number", ...messages.idNumber, masked: false },
    { key: "access_code", ...messages.accessCode, masked: true },
    { key: "ssn_last4", ...messages.ssnLast4, masked: true },
    { key: "birth_date", ...messages.birthDate, masked: false },
  ];
}

export function LoginScreen(props: {
  login: (idNumber: string, accessCode: string, ssnLast4: string, birthDate: string) => void;
  busy: boolean;
}) {
  const [values, setValues] = createSignal<Record<string, string>>({});

  function change(key: string, value: string) {
    setValues({ ...values(), [key]: value });
  }

  function submit(e: Event) {
    e.preventDefault();
    const v = values();
    props.login(v.id_number ?? "", v.access_code ?? "", v.ssn_last4 ?? "", v.birth_date ?? "");
    setValues({});
  }

  return (
    <>
      <h2>{t().authTitle}</h2>
      <form class="login" onSubmit={submit}>
        <For each={fields()}>
          {(field) => (
            <input
              type={field.masked ? "password" : "text"}
              placeholder={`${field.label} (${field.hint})`}
              value={values()[field.key] ?? ""}
              onInput={(e) => change(field.key, e.currentTarget.value)}
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
