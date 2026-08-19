import { For } from "solid-js";
import { t, locale, setLocale, type Locale } from "../i18n";
import { devMode, setDevMode } from "../devMode";
import { Toggle } from "./Toggle";
import { ToggleButton } from "./ToggleButton";
import "./Header.css";

const LOCALES: Locale[] = ["es", "en"];

export function Header() {
  return (
    <div class="header">
      <h1>
        {t().title}
      </h1>
      <div class="header-controls">
        <div class="locale-switch">
          {/* Locale switcher */}
          <For each={LOCALES}>
            {(l) => <ToggleButton label={l.toUpperCase()} active={locale() === l} onClick={() => setLocale(l)} />}
          </For>
        </div>
        <Toggle label={t().developerMode} checked={devMode()} onChange={setDevMode} />
      </div>
    </div>
  );
}
