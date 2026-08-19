import { OptionButtons } from "../components/OptionButtons";
import type { MenuOption } from "../types";

export function MainMenuScreen(props: { options: MenuOption[]; busy: boolean; onChoose: (key: string) => void }) {
  return (
    <>
      <h2>MENU PRINCIPAL</h2>
      <OptionButtons options={props.options} separator=". " busy={props.busy} onChoose={props.onChoose} />
    </>
  );
}
