import { createMemo, createSignal, Show } from "solid-js";
import { t, localizeButton } from "../i18n";
import { OptionButtons } from "../components/OptionButtons";
import { FreeTextForm } from "../components/FreeTextForm";
import { CourseTable } from "../components/CourseTable";
import type { TuiScreenComponentProps } from "../types";
import { createKeyboardListener } from "../components/KeyboardListener";

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

  function submit(text: string) {
    props.send({ kind: "Line", text });
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
        <OptionButtons options={localizedActions()} busy={props.busy} onChoose={choose} hideKey />
      </Show>

      <Show when={FREE_TEXT_MODES.has(props.screen.mode.kind)}>
        <FreeTextForm value={freeText()} onInput={setFreeText} onSubmit={submit} busy={props.busy} />
      </Show>
    </>
  );
}
