import { OptionButtons } from "../components/OptionButtons";
import type { MenuOption, Send } from "../types";

export function SelectPeriodScreen(props: { options: MenuOption[]; busy: boolean; send: Send }) {
  function choose(key: string) {
    props.send({ kind: "Select", key });
  }

  return (
    <>
      <h2>Indique Semestre</h2>
      <OptionButtons options={props.options} separator="=" busy={props.busy} onChoose={choose} />
    </>
  );
}
