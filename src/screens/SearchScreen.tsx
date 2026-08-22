import { createSignal } from "solid-js";
import { t } from "../i18n";
import type { TuiScreenComponentProps } from "../types";

export function SearchScreen(props: TuiScreenComponentProps<"Search">) {
  const [text, setText] = createSignal("");

  function submit() {
    props.send({ kind: "Line", text: text() });
    setText("");
  }

  return (
    <>
      <h2>{t().horarioMatriculaTitle}</h2>
      <p class="text-[0.85em] opacity-75">{t().searchHints[props.screen.search]}</p>
      <div class="flex justify-center">
        <input
          class="input"
          placeholder={t().sendPlaceholder}
          value={text()}
          onInput={(e) => setText(e.currentTarget.value)}
          onKeyDown={(e) => e.key === "Enter" && submit()}
        />
        <button type="button" class="btn btn-outline btn-primary" disabled={props.busy} onClick={submit}>
          {t().send}
        </button>
      </div>
    </>
  );
}
