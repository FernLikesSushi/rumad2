import { createSignal, Show } from "solid-js";
import { t } from "../i18n";
import { devMode } from "../devMode";
import { OptionButtons } from "../components/OptionButtons";
import { FreeTextForm } from "../components/FreeTextForm";
import type { MenuOption, Send } from "../types";
import "./Unknown.css";

export function UnknownScreen(props: { raw: string; options: MenuOption[]; busy: boolean; send: Send }) {
  const [freeText, setFreeText] = createSignal("");

  function choose(key: string) {
    props.send({ kind: "Select", key });
  }

  function submit(text: string) {
    props.send({ kind: "Line", text });
    setFreeText("");
  }

  return (
    <>
      <p class="hint">{t().unknownHint}</p>
      <Show when={devMode()}>
        <pre class="raw">{props.raw}</pre>
      </Show>
      <OptionButtons options={props.options} busy={props.busy} onChoose={choose} />
      <FreeTextForm value={freeText()} onInput={setFreeText} onSubmit={submit} busy={props.busy} />
    </>
  );
}
