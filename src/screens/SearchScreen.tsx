import { createSignal } from "solid-js";
import { t } from "../i18n";
import { FreeTextForm } from "../components/FreeTextForm";
import type { SearchKind, Send } from "../types";

export function SearchScreen(props: { search: SearchKind; busy: boolean; send: Send }) {
  const [text, setText] = createSignal("");

  function submit(text: string) {
    props.send({ kind: "Line", text });
    setText("");
  }

  return (
    <>
      <h2>{t().horarioMatriculaTitle}</h2>
      <p class="text-[0.85em] opacity-75">{t().searchHints[props.search]}</p>
      <FreeTextForm value={text()} onInput={setText} onSubmit={submit} busy={props.busy} />
    </>
  );
}
