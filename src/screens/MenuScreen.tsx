import { createMemo } from "solid-js";
import { t } from "../i18n";
import { OptionButtons } from "../components/OptionButtons";
import type { MenuKind, MenuOption, Send } from "../types";

export function MenuScreen(props: { menu: MenuKind; options: MenuOption[]; busy: boolean; send: Send }) {
  function choose(key: string) {
    props.send({ kind: "Select", key });
  }

  function title() {
    return props.menu === "HorarioSemester" ? t().horarioMatriculaTitle : t().menuTitles[props.menu];
  }

  // "0" (MainMenu's "SALIR DEL SISTEMA", MenuDespliegue's "Finalizar") is
  // the exit-equivalent key -- already reachable via the app's one shared
  // exit control (`canExit`, driven by the backend's own `RumadScreen::
  // exit()`), so listing it again as a regular option here would just be
  // the same action shown twice.
  const options = createMemo(() => props.options.filter((option) => option.key !== "0"));

  return (
    <>
      <h2>{title()}</h2>
      <OptionButtons options={options()} busy={props.busy} onChoose={choose} />
    </>
  );
}
