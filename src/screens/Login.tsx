import { createSignal, For } from "solid-js";
import { t } from "../i18n";
import { LogIn } from "lucide-solid";

// Hardcoded rather than read from the backend's `fields` -- the remote's
// own labels arrive mangled over the wire (see `LoginField`'s Rust-side
// doc comment), but the field set/order is fixed, so it's safe to
// hardcode and localize here. `birth_date` (a date picker) and
// `id_number` (needs live "xxx-xx-xxxx" formatting) each need their own
// handling below and are excluded here.
function textFields() {
  const messages = t().loginFields;
  return [
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

// The real ID number is 9 digits (confirmed by the remote's own hint
// text, `screens/login.rs`'s `DETECT_HINT` constant: "Ej. 802999999"),
// displayed "xxx-xx-xxxx" -- reformats on every keystroke rather than
// just masking, so pasting/backspacing/editing anywhere in the field
// still lands on a clean grouping instead of stray dashes. Re-derived
// from the raw digits each time (not inserted once), so this is
// idempotent whether `raw` came from the user typing a fresh digit or
// backspacing over a dash this same function just inserted.
function formatIdNumber(raw: string): string {
  const digits = raw.replace(/\D/g, "").slice(0, 9);
  return [digits.slice(0, 3), digits.slice(3, 5), digits.slice(5, 9)].filter(Boolean).join("-");
}

/** The login form -- see `textFields` above for why its fields are
 * hardcoded rather than scraped from the backend. */
export function LoginScreen(props: {
  login: (idNumber: string, accessCode: string, ssnLast4: string, birthDate: string) => void;
  busy: boolean;
}) {
  const [values, setValues] = createSignal<Record<string, string>>({});
  const [idNumber, setIdNumber] = createSignal("");
  const [birthDate, setBirthDate] = createSignal("");

  function change(key: string, value: string) {
    setValues({ ...values(), [key]: value });
  }

  function submit(e: Event) {
    e.preventDefault();
    const v = values();
    // Strips back to raw digits -- the dashes are display-only,
    // `LoginScreen::sanitize` (login.rs) only strips whitespace, so
    // sending them as-typed would write literal "-" characters into
    // the remote's fixed-width field instead of digits.
    props.login(idNumber().replace(/\D/g, ""), v.access_code, v.ssn_last4, toRemoteFormat(birthDate()));
    setValues({});
    setIdNumber("");
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
          {/* Label above (what the field is) plus a placeholder inside
              it (an example value) -- `type="date"` below can't do the
              same split since it ignores `placeholder` entirely
              (WebKit/Blink/Gecko all special-case it away), so its hint
              stays folded into its label instead. */}
          <div class="flex flex-col text-left md:w-full">
            <label for="id_number" class="text-xs opacity-70 mb-1 px-1">
              {t().loginFields.idNumber.label}
            </label>
            <input
              id="id_number"
              class="input md:w-full"
              type="text"
              inputmode="numeric"
              placeholder={t().loginFields.idNumber.hint}
              value={idNumber()}
              onInput={(e) => setIdNumber(formatIdNumber(e.currentTarget.value))}
            />
          </div>
          <For each={textFields()}>
            {(field) => (
              <div class="flex flex-col text-left md:w-full">
                <label for={field.key} class="text-xs opacity-70 mb-1 px-1">
                  {field.label}
                </label>
                <input
                  id={field.key}
                  class="input md:w-full"
                  type={field.masked ? "password" : "text"}
                  placeholder={field.hint}
                  value={values()[field.key] ?? ""}
                  onInput={(e) => change(field.key, e.currentTarget.value)}
                />
              </div>
            )}
          </For>
          <div class="flex flex-col text-left md:w-full">
            <label for="birth-date" class="text-xs opacity-70 mb-1 px-1">
              {t().loginFields.birthDate.label} ({t().loginFields.birthDate.hint})
            </label>
            <input
              id="birth-date"
              type="date"
              class="input md:w-full"
              value={birthDate()}
              onInput={(e) => setBirthDate(e.currentTarget.value)}
            />
          </div>
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
