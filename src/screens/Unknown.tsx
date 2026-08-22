import { createSignal, Show } from "solid-js";
import { t } from "../i18n";
import { devMode } from "../devMode";
import { OptionButtons } from "../components/OptionButtons";
import { FreeTextForm } from "../components/FreeTextForm";
import { createKeyboardListener } from "../components/KeyboardListener";
import type { TuiScreenComponentProps } from "../types";

export function UnknownScreen(props: TuiScreenComponentProps<"Unknown">) {
  const [freeText, setFreeText] = createSignal("");

  function choose(key: string) {
    props.send({ kind: "Select", key });
  }

  createKeyboardListener((key) => {
    if (props.screen.options.some((option) => option.key === key)) {
      choose(key);
    }
  });

  function submit(text: string) {
    props.send({ kind: "Line", text });
    setFreeText("");
  }

  return (
    <>
      <p class="text-[0.85em] opacity-75">{t().unknownHint}</p>
      <Show when={devMode()}>
        <pre class="bg-base-200 p-3 rounded-box overflow-x-auto text-sm whitespace-pre">{props.screen.raw}</pre>
      </Show>
      <OptionButtons options={props.screen.options} busy={props.busy} onChoose={choose} />
      <FreeTextForm value={freeText()} onInput={setFreeText} onSubmit={submit} busy={props.busy} />
    </>
  );
}
