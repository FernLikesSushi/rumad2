import { t } from "../i18n";
import { OptionButtons } from "../components/OptionButtons";
import { FreeTextForm } from "../components/FreeTextForm";
import type { MenuOption } from "../types";

export function UnknownScreen(props: {
  raw: string;
  options: MenuOption[];
  busy: boolean;
  onChoose: (key: string) => void;
  freeText: string;
  onFreeTextInput: (value: string) => void;
  onFreeTextSubmit: (e: Event) => void;
}) {
  return (
    <>
      <p class="hint">{t().unknownHint}</p>
      <pre class="raw">{props.raw}</pre>
      <OptionButtons options={props.options} separator=". " busy={props.busy} onChoose={props.onChoose} />
      <FreeTextForm
        value={props.freeText}
        onInput={props.onFreeTextInput}
        onSubmit={props.onFreeTextSubmit}
        busy={props.busy}
      />
    </>
  );
}
