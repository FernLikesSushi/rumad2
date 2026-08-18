import { createSignal, Match, Switch, For, Show } from "solid-js";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

type MenuOption = { key: string; label: string };

type TuiScreen =
  | { kind: "MainMenu"; options: MenuOption[] }
  | { kind: "Notice"; message: string; raw: string }
  | { kind: "Disconnected" }
  | { kind: "Unknown"; raw: string; options: MenuOption[] };

type Dialog = { title: string; message: string };

function App() {
  const [screen, setScreen] = createSignal<TuiScreen | null>(null);
  const [username, setUsername] = createSignal("");
  const [password, setPassword] = createSignal("");
  const [freeText, setFreeText] = createSignal("");
  const [busy, setBusy] = createSignal(false);
  const [dialog, setDialog] = createSignal<Dialog | null>(null);

  async function run<T>(action: () => Promise<T>) {
    setBusy(true);
    try {
      return await action();
    } catch (e) {
      setDialog({ title: "Error", message: String(e) });
      return undefined;
    } finally {
      setBusy(false);
    }
  }

  // `Notice` is a message alongside the current screen, not a screen of
  // its own -- surface it as a dialog on top of whatever's showing rather
  // than replacing it.
  function applyScreen(result: TuiScreen | undefined) {
    if (!result) return;
    if (result.kind === "Notice") {
      setDialog({ title: "Aviso", message: result.message });
      return;
    }
    setScreen(result);
  }

  async function connect() {
    const result = await run(() =>
      invoke<TuiScreen>("connect", {
        username: username() || undefined,
        password: password() || undefined,
      }),
    );
    applyScreen(result);
  }

  async function chooseOption(key: string) {
    const result = await run(() => invoke<TuiScreen>("send_input", { text: key }));
    applyScreen(result);
  }

  async function sendFreeText(e: Event) {
    e.preventDefault();
    const result = await run(() => invoke<TuiScreen>("send_input", { text: freeText() }));
    applyScreen(result);
    setFreeText("");
  }

  async function disconnect() {
    await run(() => invoke("disconnect"));
    setScreen(null);
  }

  return (
    <main class="container">
      <h1>
        RUMAD
        <Show when={busy()}>
          <span class="spinner" role="status" aria-label="Cargando" />
        </Show>
      </h1>

      <Show when={dialog()}>
        {(d) => (
          <div class="dialog-overlay" onClick={() => setDialog(null)}>
            <div class="dialog" role="alertdialog" aria-modal="true" onClick={(e) => e.stopPropagation()}>
              <h3>{d().title}</h3>
              <p>{d().message}</p>
              <button onClick={() => setDialog(null)}>Cerrar</button>
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
            <p class="hint">
              Leave blank to use the shared "estudiante" demo login (main
              menu only). Enter your own RUMAD credentials to reach
              account-specific screens.
            </p>
            <input
              placeholder="Usuario / ID de estudiante"
              value={username()}
              onInput={(e) => setUsername(e.currentTarget.value)}
            />
            <input
              type="password"
              placeholder="Clave de acceso"
              value={password()}
              onInput={(e) => setPassword(e.currentTarget.value)}
            />
            <button type="submit" disabled={busy()}>
              {busy() ? "Conectando..." : "Conectar"}
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
              <h2>Sesion finalizada</h2>
              <p class="hint">
                El sistema remoto termino la sesion (PROCESO CONCLUIDO).
              </p>
              <button disabled={busy()} onClick={() => setScreen(null)}>
                Reconectar
              </button>
            </Match>

            <Match when={screen()?.kind === "Unknown"}>
              <p class="hint">Pantalla sin reconocer todavia:</p>
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
                  placeholder="Enviar texto..."
                  value={freeText()}
                  onInput={(e) => setFreeText(e.currentTarget.value)}
                />
                <button type="submit" disabled={busy()}>
                  Enviar
                </button>
              </form>
            </Match>
          </Switch>

          <Show when={screen()?.kind !== "Disconnected"}>
            <button class="disconnect" disabled={busy()} onClick={disconnect}>
              Salir
            </button>
          </Show>
        </div>
      </Show>
    </main>
  );
}

export default App;
