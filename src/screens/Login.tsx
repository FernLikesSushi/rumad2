import { createSignal, For } from "solid-js";
import { t } from "../i18n";
import type { Send } from "../types";

// Hardcoded rather than read from the backend's `fields` -- the remote's
// own labels for this specific form arrive mangled over the wire (see
// `LoginField`'s doc comment on the Rust side, which hardcodes the same
// text for the same reason), and the field set/order is fixed regardless.
// Unlike other remote-mirroring text, this is safe to localize since it's
// a fixed, known set rather than open-ended scraped content -- see
// `Messages.loginFields`'s doc comment. `birth_date` is handled separately
// below (a date picker, not a text input), so it's excluded here.
function textFields() {
  const messages = t().loginFields;
  return [
    { key: "id_number", ...messages.idNumber, masked: false },
    { key: "access_code", ...messages.accessCode, masked: true },
    { key: "ssn_last4", ...messages.ssnLast4, masked: true },
  ];
}

// The remote expects this field as 8 digits with no separators
// (MMDDAAAA, per `LoginField`'s own "Ej. MMDDAAAA" hint on the Rust
// side) -- a native date picker is friendlier than asking the user to
// type that format themselves, so this converts the picker's ISO
// "YYYY-MM-DD" value at submit time instead.
function toRemoteFormat(isoDate: string): string {
  const [year, month, day] = isoDate.split("-");
  return `${month}${day}${year}`;
}

export function LoginScreen(props: {
  login: (idNumber: string, accessCode: string, ssnLast4: string, birthDate: string) => void;
  send: Send;
  busy: boolean;
}) {
  const [values, setValues] = createSignal<Record<string, string>>({});
  const [birthDate, setBirthDate] = createSignal("");

  function change(key: string, value: string) {
    setValues({ ...values(), [key]: value });
  }

  function submit(e: Event) {
    e.preventDefault();
    const v = values();
    props.login(v.id_number, v.access_code, v.ssn_last4, toRemoteFormat(birthDate()));
    setValues({});
    setBirthDate("");
  }

  // `LoginScreen`'s own `RumadScreen::exit` sends PF4 (its footer's own
  // "PF4=(9)" hint, live-confirmed) rather than the "0" every other
  // screen's default `exit` sends -- see the backend impl's doc comment.
  function exitScreen() {
    props.send({ kind: "Exit" });
  }

  return (
    <>
      <h2>{t().authTitle}</h2>
      <form class="login" onSubmit={submit}>
        <For each={textFields()}>
          {(field) => (
            <input
              type={field.masked ? "password" : "text"}
              placeholder={`${field.label} (${field.hint})`}
              value={values()[field.key] ?? ""}
              onInput={(e) => change(field.key, e.currentTarget.value)}
            />
          )}
        </For>
        <input
          type="date"
          aria-label={t().loginFields.birthDate.label}
          value={birthDate()}
          onInput={(e) => setBirthDate(e.currentTarget.value)}
        />
        <div class="row">
          <button type="submit" disabled={props.busy}>
            {t().send}
          </button>
          <button type="button" disabled={props.busy} onClick={exitScreen}>
            {t().screenExit}
          </button>
        </div>
      </form>
    </>
  );
}
