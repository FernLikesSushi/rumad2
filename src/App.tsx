import { createSignal, createResource, createMemo, createEffect, Match, Switch, For, Show } from "solid-js";
import { invoke } from "@tauri-apps/api/core";
import { t, locale, setLocale, type Locale } from "./i18n";
import "./App.css";

type MenuOption = { key: string; label: string };

type TuiScreen =
  | { kind: "MainMenu"; options: MenuOption[] }
  | { kind: "Notice"; message: string; raw: string }
  | { kind: "Disconnected" }
  | { kind: "Unknown"; raw: string; options: MenuOption[] };

type Dialog = { title: string; message: string };

type Action =
  | { cmd: "connect"; args: { username?: string; password?: string } }
  | { cmd: "send_input"; args: { text: string } }
  | { cmd: "disconnect"; args: {} };

// The resource's fetcher: `null` means "session ended",
// anything else is whatever the invoked command
// resolved to.
async function runAction(action: Action): Promise<TuiScreen | null> {
  if (action.cmd === "disconnect") {
    await invoke("disconnect");
    return null;
  }
  return invoke<TuiScreen>(action.cmd, action.args);
}

const LOCALES: Locale[] = ["es", "en"];

function App() {
  const [username, setUsername] = createSignal("");
  const [password, setPassword] = createSignal("");
  const [freeText, setFreeText] = createSignal("");
  const [dialog, setDialog] = createSignal<Dialog | null>(null);

  const [action, setAction] = createSignal<Action>();
  const [response, { mutate }] = createResource(action, runAction);
  const busy = () => response.loading;

  // What to render is derived, not imperatively assigned: `Notice` is a
  // message alongside the current screen rather than a screen change, so
  // the memo keeps whatever was showing instead of switching to it.
  const screen = createMemo<TuiScreen | null>((prev) => {
    if (response.loading) return prev ?? null;
    const result = response();
    if (result === undefined || result?.kind === "Notice") return prev ?? null;
    return result;
  }, null);

  // Opening a dialog is a genuine side effect (and independently
  // dismissible via the close button), unlike `screen` above -- it
  // belongs in an effect, not a memo.
  createEffect(() => {
    if (response.loading) return;
    const err = response.error;
    if (err) {
      setDialog({ title: t().errorTitle, message: String(err) });
      return;
    }
    const result = response();
    if (result?.kind === "Notice") {
      setDialog({ title: t().noticeTitle, message: result.message });
    }
  });

  function connect() {
    setAction({
      cmd: "connect",
      args: { username: username() || undefined, password: password() || undefined },
    });
  }

  function chooseOption(key: string) {
    setAction({ cmd: "send_input", args: { text: key } });
  }

  function sendFreeText(e: Event) {
    e.preventDefault();
    setAction({ cmd: "send_input", args: { text: freeText() } });
    setFreeText("");
  }

  function disconnect() {
    setAction({ cmd: "disconnect", args: {} });
  }

  // The remote already closed the session server-side, so this just
  // overrides the resource's resolved value locally instead of firing a
  // command -- see `mutate`'s docs for why that's the right tool here.
  function reconnect() {
    mutate(null);
  }

  return (
    <main class="container">
      <div class="header">
        <h1>
          {t().title}
          <Show when={busy()}>
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

      <Show when={dialog()}>
        {(d) => (
          <div class="dialog-overlay" onClick={() => setDialog(null)}>
            <div class="dialog" role="alertdialog" aria-modal="true" onClick={(e) => e.stopPropagation()}>
              <h3>{d().title}</h3>
              <p>{d().message}</p>
              <button onClick={() => setDialog(null)}>{t().close}</button>
            </div>
          </div>
        )}
      </Show>

      <Show
        when={screen()}
        fallback={
          <form
            class="login"
            onSubmit={(e) => {
              e.preventDefault();
              connect();
            }}
          >
            <p class="hint">{t().loginHint}</p>
            <input
              placeholder={t().usernamePlaceholder}
              value={username()}
              onInput={(e) => setUsername(e.currentTarget.value)}
            />
            <input
              type="password"
              placeholder={t().passwordPlaceholder}
              value={password()}
              onInput={(e) => setPassword(e.currentTarget.value)}
            />
            <button type="submit" disabled={busy()}>
              {busy() ? t().connecting : t().connect}
            </button>
          </form>
        }
      >
        <div class="screen">
          <Switch>
            <Match when={screen()?.kind === "MainMenu"}>
              <h2>MENU PRINCIPAL</h2>
              <div class="options">
                <For each={(screen() as Extract<TuiScreen, { kind: "MainMenu" }>).options}>
                  {(option) => (
                    <button disabled={busy()} onClick={() => chooseOption(option.key)}>
                      {option.key}. {option.label}
                    </button>
                  )}
                </For>
              </div>
            </Match>

            <Match when={screen()?.kind === "Disconnected"}>
              <h2>{t().disconnectedTitle}</h2>
              <p class="hint">{t().disconnectedHint}</p>
              <button disabled={busy()} onClick={reconnect}>
                {t().reconnect}
              </button>
            </Match>

            <Match when={screen()?.kind === "Unknown"}>
              <p class="hint">{t().unknownHint}</p>
              <pre class="raw">{(screen() as Extract<TuiScreen, { kind: "Unknown" }>).raw}</pre>
              <div class="options">
                <For each={(screen() as Extract<TuiScreen, { kind: "Unknown" }>).options}>
                  {(option) => (
                    <button disabled={busy()} onClick={() => chooseOption(option.key)}>
                      {option.key}. {option.label}
                    </button>
                  )}
                </For>
              </div>
              <form class="row" onSubmit={sendFreeText}>
                <input
                  placeholder={t().sendPlaceholder}
                  value={freeText()}
                  onInput={(e) => setFreeText(e.currentTarget.value)}
                />
                <button type="submit" disabled={busy()}>
                  {t().send}
                </button>
              </form>
            </Match>
          </Switch>

          <Show when={screen()?.kind !== "Disconnected"}>
            <button class="disconnect" disabled={busy()} onClick={disconnect}>
              {t().logout}
            </button>
          </Show>
        </div>
      </Show>
    </main>
  );
}

export default App;
