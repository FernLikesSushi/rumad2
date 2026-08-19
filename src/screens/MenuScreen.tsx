import { t } from "../i18n";
import { OptionButtons } from "../components/OptionButtons";
import type { MenuKind, MenuOption, Send } from "../types";

// MainMenu/MenuDespliegue use the remote's own numbered ". " style;
// SelectPeriod/HorarioSemester use its "=" style instead.
const EQUALS_SEPARATOR_KINDS = new Set<MenuKind>(["SelectPeriod", "HorarioSemester"]);

export function MenuScreen(props: { menu: MenuKind; options: MenuOption[]; busy: boolean; send: Send }) {
  function choose(key: string) {
    props.send({ kind: "Select", key });
  }

  function title() {
    return props.menu === "HorarioSemester" ? t().horarioMatriculaTitle : t().menuTitles[props.menu];
  }

  return (
    <>
      <h2>{title()}</h2>
      <OptionButtons
        options={props.options}
        separator={EQUALS_SEPARATOR_KINDS.has(props.menu) ? "=" : ". "}
        busy={props.busy}
        onChoose={choose}
      />
    </>
  );
}
