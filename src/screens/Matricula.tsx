import { For, Show } from "solid-js";
import { OptionButtons } from "../components/OptionButtons";
import { FreeTextForm } from "../components/FreeTextForm";
import type { ScheduleCourse, MatriculaPrompt } from "../types";

// Bajas/Altas/Cambio all show the same "course abbreviation, or FIN" free-
// text prompt -- only the [Bajas]/[Altas]/[Cambio] tag differs on-screen.
const FREE_TEXT_PROMPTS = new Set(["Bajas", "Altas", "Cambio"]);

export function MatriculaScreen(props: {
  courses: ScheduleCourse[];
  prompt: MatriculaPrompt;
  busy: boolean;
  onChoose: (key: string) => void;
  freeText: string;
  onFreeTextInput: (value: string) => void;
  onFreeTextSubmit: (e: Event) => void;
}) {
  return (
    <>
      <h2>M A T R I C U L A</h2>
      <table class="courses">
        <thead>
          <tr>
            <th>Curso</th>
            <th>Seccion</th>
            <th>Cr.</th>
            <th>Grado</th>
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

      <Show when={props.prompt.kind === "Actions"}>
        <OptionButtons
          options={(props.prompt as Extract<MatriculaPrompt, { kind: "Actions" }>).options}
          separator="="
          busy={props.busy}
          onChoose={props.onChoose}
        />
      </Show>

      <Show when={FREE_TEXT_PROMPTS.has(props.prompt.kind)}>
        <FreeTextForm
          value={props.freeText}
          onInput={props.onFreeTextInput}
          onSubmit={props.onFreeTextSubmit}
          busy={props.busy}
        />
      </Show>
    </>
  );
}
