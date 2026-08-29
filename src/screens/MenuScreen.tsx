import { createMemo } from "solid-js";
import { t, localizeButton } from "../i18n";
import { OptionButtons } from "../components/OptionButtons";
import type { TuiScreenComponentProps } from "./api";
import { createKeyboardListener } from "../components/KeyboardListener";

/** A numbered/lettered menu (MainMenu, MenuDespliegue, SelectPeriod,
 * HorarioSemester) -- one option per remote menu line, picked by key. */
export function MenuScreen(props: TuiScreenComponentProps<"Menu">) {
  function choose(key: string) {
    props.send({ kind: "Select", key });
  }

  function title() {
    return props.screen.menu === "HorarioSemester" ? t().horarioMatriculaTitle : t().menuTitles[props.screen.menu];
  }

  // "0" (exit-equivalent, e.g. "SALIR DEL SISTEMA") is dropped -- already
  // reachable via the app's one shared exit control (`canExit`).
  const options = createMemo(() =>
    props.screen.options
      .filter((option) => option.key !== "0")
      .map((option) => ({ ...option, label: localizeButton(option.label) })),
  );

  createKeyboardListener((key) => {
    if (options().some((option) => option.key === key)) {
      choose(key);
    }
  });

  return (
    <>
      <h2>{title()}</h2>
      <OptionButtons options={options()} busy={props.busy} onChoose={choose} menu={props.screen.menu} />
    </>
  );
}
