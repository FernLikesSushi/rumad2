import { createSignal } from "solid-js";
import { t } from "../i18n";
import type { TuiScreenComponentProps } from "./api";
import { Send } from "lucide-solid";

/** Free-text course-code search prompt (HorarioCurso/HorarioSeccion). */
export function SearchScreen(props: TuiScreenComponentProps<"Search">) {
  const [text, setText] = createSignal("");

  function submit() {
    props.send({ kind: "Line", text: text() });
    setText("");
  }

  return (
    <div class="max-w-lg self-center flex flex-col justify-center">
      <h2>{t().horarioMatriculaTitle}</h2>
      <p class="text-[0.85em] opacity-75 text-center">{t().searchHints[props.screen.search]}</p>
      <div class="flex flex-col justify-center gap-4">
        <input
          class="input w-full"
          placeholder={t().sendPlaceholder}
          value={text()}
          onInput={(e) => setText(e.currentTarget.value)}
          onKeyDown={(e) => e.key === "Enter" && submit()}
        />
        <button type="button" class="btn btn-outline btn-primary" disabled={props.busy} onClick={submit}>
          {t().send}
          <Send />
        </button>
      </div>
    </div>
  );
}
