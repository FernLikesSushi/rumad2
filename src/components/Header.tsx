import { For, Show } from "solid-js";
import { t, locale, setLocale, type Locale } from "../i18n";

const LOCALES: Locale[] = ["es", "en"];

export function Header(props: { busy: boolean }) {
  return (
    <div class="header">
      <h1>
        {t().title}
        <Show when={props.busy}>
          <span class="spinner" role="status" aria-label={t().loading} />
        </Show>
      </h1>
      <div class="locale-switch">
        <For each={LOCALES}>
          {(l) => (
            <button classList={{ active: locale() === l }} onClick={() => setLocale(l)}>
              {l.toUpperCase()}
            </button>
          )}
        </For>
      </div>
    </div>
  );
}
