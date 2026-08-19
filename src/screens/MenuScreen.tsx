import { createMemo } from "solid-js";
import { t, localizeButton } from "../i18n";
import { OptionButtons } from "../components/OptionButtons";
import type { MenuKind, MenuOption, Send } from "../types";
import { createKeyboardListener } from "../components/KeyboardListener";

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
  // the same action shown twice. Labels are localized via `localizeButton`
  // (matches `HorarioSemester`'s compact codes; every other menu's labels
  // simply fall back to their raw text unchanged).
  const options = createMemo(() =>
    props.options
      .filter((option) => option.key !== "0")
      .map((option) => ({ ...option, label: localizeButton(option.label) })),
  );

  // Keyboard listener for menu option selection
  createKeyboardListener((key) => {
    if (options().some((option) => option.key === key)) {
      choose(key);
    }
  });

  return (
    <>
      <h2>{title()}</h2>
      <OptionButtons options={options()} busy={props.busy} onChoose={choose} />
    </>
  );
}
