import { For } from "solid-js";
import type { MenuOption } from "../types";

export function OptionButtons(props: {
  options: MenuOption[];
  separator: string;
  busy: boolean;
  onChoose: (key: string) => void;
}) {
  return (
    <div class="options">
      <For each={props.options}>
        {(option) => (
          <button disabled={props.busy} onClick={() => props.onChoose(option.key)}>
            {option.key}
            {props.separator}
            {option.label}
          </button>
        )}
      </For>
    </div>
  );
}
