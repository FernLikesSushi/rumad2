import { createMemo, createSignal, Show } from "solid-js";
import { t, localizeButton } from "../../i18n";
import { OptionButtons } from "../../components/OptionButtons";
import { CourseTable } from "./CourseTable";
import type { TuiScreenComponentProps } from "../api";
import { createKeyboardListener } from "../../components/KeyboardListener";

// Bajas/Altas/Cambio all show the same "course abbreviation, or FIN" free-
// text prompt -- only the [Bajas]/[Altas]/[Cambio] tag differs on-screen.
const FREE_TEXT_MODES = new Set(["Bajas", "Altas", "Cambio"]);

export function MatriculaScreen(props: TuiScreenComponentProps<"Matricula">) {
  const [freeText, setFreeText] = createSignal("");

  // Only the "Actions" mode actually carries `options` -- Bajas/Altas/
  // Cambio don't, and the keyboard listener below reads this on every
  // keypress regardless of the current mode, so this must stay a real
  // array (not undefined) even outside "Actions".
  const options = createMemo(() => (props.screen.mode.kind === "Actions" ? props.screen.mode.options : []));

  // Keyboard listener for menu option selection
  createKeyboardListener((key) => {
    if (options().some((option) => option.key === key)) {
      choose(key);
    }
  });

  function choose(key: string) {
    props.send({ kind: "Select", key });
  }

  function submit() {
    props.send({ kind: "Line", text: freeText() });
    setFreeText("");
  }

  // Actions' labels are the remote's own raw text (e.g. "HorEst",
  // "CodigoReservar") -- localize via `localizeButton`, falling back to
  // the raw label for anything not listed there.
  function localizedActions() {
    return options().map((option) => ({ ...option, label: localizeButton(option.label) }));
  }

  return (
    <>
      <h2>{t().matriculaTitle}</h2>
      <CourseTable courses={props.screen.courses} />

      <Show when={props.screen.mode.kind === "Actions"}>
        <OptionButtons options={localizedActions()} busy={props.busy} onChoose={choose} menu="Matricula" hideKey />
      </Show>

      <Show when={FREE_TEXT_MODES.has(props.screen.mode.kind)}>
        <div class="flex justify-center">
          <input
            class="input"
            placeholder={t().sendPlaceholder}
            value={freeText()}
            onInput={(e) => setFreeText(e.currentTarget.value)}
            onKeyDown={(e) => e.key === "Enter" && submit()}
          />
          <button type="button" class="btn btn-outline btn-primary" disabled={props.busy} onClick={submit}>
            {t().send}
          </button>
        </div>
      </Show>
    </>
  );
}
