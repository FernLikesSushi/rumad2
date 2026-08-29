import { createSignal, For } from "solid-js";
import { t } from "../i18n";
import { LogIn } from "lucide-solid";

// Hardcoded rather than read from the backend's `fields` -- the remote's
// own labels arrive mangled over the wire (see `LoginField`'s Rust-side
// doc comment), but the field set/order is fixed, so it's safe to
// hardcode and localize here. `birth_date` is a date picker, not a text
// input, so it's handled separately below and excluded here.
function textFields() {
  const messages = t().loginFields;
  return [
    { key: "id_number", ...messages.idNumber, masked: false },
    { key: "access_code", ...messages.accessCode, masked: true },
    { key: "ssn_last4", ...messages.ssnLast4, masked: true },
  ];
}

// The remote expects this field as 8 digits, MMDDAAAA -- converts the
// date picker's ISO "YYYY-MM-DD" value at submit time instead of asking
// the user to type that format themselves.
function toRemoteFormat(isoDate: string): string {
  const [year, month, day] = isoDate.split("-");
  return `${month}${day}${year}`;
}

/** The login form -- see `textFields` above for why its fields are
 * hardcoded rather than scraped from the backend. */
export function LoginScreen(props: {
  login: (idNumber: string, accessCode: string, ssnLast4: string, birthDate: string) => void;
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

  return (
    <>
      <h2>{t().authTitle}</h2>
      <div class="flex flex-col items-center">
        <form
          class="flex flex-col md:grid md:grid-cols-2 gap-2.5 max-w-lg md:max-w-2xl items-center md:items-stretch text-center"
          onSubmit={submit}
        >
          <For each={textFields()}>
            {(field) => (
              <input
                class="input md:w-full"
                type={field.masked ? "password" : "text"}
                placeholder={`${field.label} (${field.hint})`}
                value={values()[field.key] ?? ""}
                onInput={(e) => change(field.key, e.currentTarget.value)}
              />
            )}
          </For>
          <input
            type="date"
            class="input md:w-full"
            aria-label={t().loginFields.birthDate.label}
            value={birthDate()}
            onInput={(e) => setBirthDate(e.currentTarget.value)}
          />
          <div class="flex justify-center md:col-span-2 text">
            <p class="text-sm text-muted-foreground">{t().loginPrivacyNotice}</p>
          </div>
          <div class="flex justify-center md:col-span-2">
            <button type="submit" class="btn btn-outline btn-primary gap-2" disabled={props.busy}>
              {t().login}
              <LogIn />
            </button>
          </div>
        </form>
      </div>
    </>
  );
}
