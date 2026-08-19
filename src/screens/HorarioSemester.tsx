import { t } from "../i18n";
import { OptionButtons } from "../components/OptionButtons";
import type { MenuOption, Send } from "../types";

export function HorarioSemesterScreen(props: { options: MenuOption[]; busy: boolean; send: Send }) {
  function choose(key: string) {
    props.send({ kind: "Select", key });
  }

  // Unlike SelectPeriod's own listed "S=salir" option, this prompt has no
  // exit option among its four choices -- PF4 is the only way out (see
  // `RumadScreen for HorarioSemesterScreen` in the backend).
  function exitScreen() {
    props.send({ kind: "Exit" });
  }

  return (
    <>
      <h2>{t().horarioMatriculaTitle}</h2>
      <OptionButtons options={props.options} separator="=" busy={props.busy} onChoose={choose} />
      <button disabled={props.busy} onClick={exitScreen}>
        {t().screenExit}
      </button>
    </>
  );
}
