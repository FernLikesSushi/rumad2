import { t } from "../i18n";
import { OptionButtons } from "../components/OptionButtons";
import type { MenuOption, Send } from "../types";

export function MainMenuScreen(props: { options: MenuOption[]; busy: boolean; send: Send }) {
  function choose(key: string) {
    props.send({ kind: "Select", key });
  }

  return (
    <>
      <h2>{t().mainMenuTitle}</h2>
      <OptionButtons options={props.options} separator=". " busy={props.busy} onChoose={choose} />
    </>
  );
}
