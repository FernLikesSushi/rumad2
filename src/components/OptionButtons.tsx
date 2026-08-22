import { For, Show } from "solid-js";
import type { MenuOption } from "../types";
import { iconFor, type IconMenu } from "../data/menuIcons";

export function OptionButtons(props: {
  options: MenuOption[];
  busy: boolean;
  onChoose: (key: string) => void;
  // Which screen/menu these options belong to, so an option's icon (if
  // any) can be looked up per key -- see `iconFor`.
  menu: IconMenu;
  // Numbered menus (MainMenu, SelectPeriod, Unknown's scraped options)
  // show their key as a small hint since it's how the remote itself numbers
  // them, but a single-letter action code (Matricula's Actions prompt:
  // A/B/C/H/...) isn't meaningful on its own -- just the full word there.
  hideKey?: boolean;
}) {
  return (
    <div class="sm:flex sm:flex-col md:grid md:grid-cols-2 gap-2">
      <For each={props.options}>
        {(option) => (
          <button class="btn justify-start gap-2" disabled={props.busy} onClick={() => props.onChoose(option.key)}>
            <Show when={!props.hideKey}>
              <span class="text-xs opacity-60 shrink-0">{option.key}</span>
            </Show>
            {iconFor(props.menu, option.key)}
            <span>{option.label}</span>
          </button>
        )}
      </For>
    </div>
  );
}
