import { createSignal } from "solid-js";
import { t } from "../i18n";
import { FreeTextForm } from "../components/FreeTextForm";
import type { Send } from "../types";

export function HorarioCursoScreen(props: { busy: boolean; send: Send }) {
  const [text, setText] = createSignal("");

  function submit(text: string) {
    props.send({ kind: "Line", text });
    setText("");
  }

  function exitScreen() {
    props.send({ kind: "Exit" });
  }

  return (
    <>
      <h2>{t().horarioMatriculaTitle}</h2>
      <p class="hint">{t().horarioCursoHint}</p>
      <FreeTextForm value={text()} onInput={setText} onSubmit={submit} busy={props.busy} />
      <button disabled={props.busy} onClick={exitScreen}>
        {t().screenExit}
      </button>
    </>
  );
}
