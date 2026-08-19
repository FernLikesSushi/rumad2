import { For, Show } from "solid-js";
import { t, locale, setLocale, type Locale } from "../i18n";
import { devMode, setDevMode } from "../devMode";
import { Spinner } from "./Spinner";
import { Toggle } from "./Toggle";
import { ToggleButton } from "./ToggleButton";
import "./Header.css";

const LOCALES: Locale[] = ["es", "en"];

export function Header(props: { busy: boolean }) {
  return (
    <div class="header">
      <h1>
        {t().title}
        <Show when={props.busy}>
          <Spinner />
        </Show>
      </h1>
      <div class="header-controls">
        <div class="locale-switch">
          <For each={LOCALES}>
            {(l) => <ToggleButton label={l.toUpperCase()} active={locale() === l} onClick={() => setLocale(l)} />}
          </For>
        </div>
        <Toggle label={t().developerMode} checked={devMode()} onChange={setDevMode} />
      </div>
    </div>
  );
}
