import { createSignal, For, Show } from "solid-js";
import { t } from "../i18n";
import { OptionButtons } from "../components/OptionButtons";
import { FreeTextForm } from "../components/FreeTextForm";
import type { ScheduleCourse, MatriculaMode, Send } from "../types";

// Bajas/Altas/Cambio all show the same "course abbreviation, or FIN" free-
// text prompt -- only the [Bajas]/[Altas]/[Cambio] tag differs on-screen.
const FREE_TEXT_MODES = new Set(["Bajas", "Altas", "Cambio"]);

export function MatriculaScreen(props: {
  courses: ScheduleCourse[];
  mode: MatriculaMode;
  busy: boolean;
  send: Send;
}) {
  const [freeText, setFreeText] = createSignal("");

  function choose(key: string) {
    props.send({ kind: "Select", key });
  }

  function submit(text: string) {
    props.send({ kind: "Line", text });
    setFreeText("");
  }

  // Actions' labels are the remote's own raw text (e.g. "HorEst",
  // "CodigoReservar") -- localize via the catalog's `actionLabels` lookup,
  // falling back to the raw label for anything not listed there.
  function localizedActions() {
    const { options } = props.mode as Extract<MatriculaMode, { kind: "Actions" }>;
    return options.map((option) => ({ ...option, label: t().actionLabels[option.label] ?? option.label }));
  }

  return (
    <>
      <h2>{t().matriculaTitle}</h2>
      <table class="courses">
        <thead>
          <tr>
            <th>{t().matriculaColumns.course}</th>
            <th>{t().matriculaColumns.section}</th>
            <th>{t().matriculaColumns.credits}</th>
            <th>{t().matriculaColumns.status}</th>
          </tr>
        </thead>
        <tbody>
          <For each={props.courses}>
            {(c) => (
              <tr>
                <td>{c.course}</td>
                <td>{c.section}</td>
                <td>{c.credits}</td>
                <td>{c.status}</td>
              </tr>
            )}
          </For>
        </tbody>
      </table>

      <Show when={props.mode.kind === "Actions"}>
        <OptionButtons options={localizedActions()} busy={props.busy} onChoose={choose} hideKey />
      </Show>

      <Show when={FREE_TEXT_MODES.has(props.mode.kind)}>
        <FreeTextForm value={freeText()} onInput={setFreeText} onSubmit={submit} busy={props.busy} />
      </Show>
    </>
  );
}
