import { For, Show } from "solid-js";
import type { MenuOption } from "../types";
import "./OptionButtons.css";

export function OptionButtons(props: {
  options: MenuOption[];
  busy: boolean;
  onChoose: (key: string) => void;
  // Numbered menus (MainMenu, SelectPeriod, Unknown's scraped options)
  // show their key as a small hint since it's how the remote itself numbers
  // them, but a single-letter action code (Matricula's Actions prompt:
  // A/B/C/H/...) isn't meaningful on its own -- just the full word there.
  hideKey?: boolean;
}) {
  return (
    <div class="options">
      <For each={props.options}>
        {(option) => (
          <button disabled={props.busy} onClick={() => props.onChoose(option.key)}>
            <Show when={!props.hideKey}>
              <span class="option-hint">{option.key}</span>
            </Show>
            <span class="option-label">{option.label}</span>
          </button>
        )}
      </For>
    </div>
  );
}
