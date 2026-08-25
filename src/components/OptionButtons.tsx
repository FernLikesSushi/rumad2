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
    <div class="flex flex-col md:grid md:grid-cols-2 gap-4">
      <For each={props.options}>
        {(option) => (
          <button
            class="btn justify-start gap-2 max-sm:h-auto max-sm:min-h-10 max-sm:py-2"
            disabled={props.busy}
            onClick={() => props.onChoose(option.key)}
          >
            <Show when={!props.hideKey}>
              <span class="text-md md:text-xs opacity-60 shrink-0">{option.key}</span>
            </Show>
            {iconFor(props.menu, option.key)}
            <span class="text-[1.15rem] md:text-md text-left min-w-0">{option.label}</span>
          </button>
        )}
      </For>
    </div>
  );
}
