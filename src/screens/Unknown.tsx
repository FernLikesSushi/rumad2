import { createSignal, Show } from "solid-js";
import { t } from "../i18n";
import { devMode } from "../data/devMode";
import { OptionButtons } from "../components/OptionButtons";
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

  function submit() {
    props.send({ kind: "Line", text: freeText() });
    setFreeText("");
  }

  return (
    <>
      <p class="text-[0.85em] opacity-75">{t().unknownHint}</p>
      <Show when={devMode()}>
        <pre class="bg-base-200 p-3 rounded-box overflow-x-auto text-sm whitespace-pre">{props.screen.raw}</pre>
      </Show>
      <OptionButtons options={props.screen.options} busy={props.busy} onChoose={choose} menu="Unknown" />
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
    </>
  );
}
