import { createSignal, createResource, createMemo, createEffect, Match, Switch, Show } from "solid-js";
import { t } from "./i18n";
import { runAction } from "./api";
import type { TuiScreen, Dialog, Action } from "./types";
import { Header } from "./components/Header";
import { NoticeDialog } from "./components/NoticeDialog";
import { ConnectForm } from "./screens/ConnectForm";
import { MainMenuScreen } from "./screens/MainMenu";
import { LoginScreen } from "./screens/Login";
import { SelectPeriodScreen } from "./screens/SelectPeriod";
import { MatriculaScreen } from "./screens/Matricula";
import { DisconnectedScreen } from "./screens/Disconnected";
import { UnknownScreen } from "./screens/Unknown";
import "./App.css";

function App() {
  const [username, setUsername] = createSignal("");
  const [password, setPassword] = createSignal("");
  const [freeText, setFreeText] = createSignal("");
  const [loginValues, setLoginValues] = createSignal<Record<string, string>>({});
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

  // `Matricula`'s prompt is nested one level deeper than the other screen
  // kinds, which makes repeated inline `as Extract<...>` casts unwieldy --
  // narrow it once here instead.
  const matricula = createMemo(() => {
    const s = screen();
    return s?.kind === "Matricula" ? s : undefined;
  });

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

  // Numbered/lettered menu options read a single keystroke with no Enter
  // -- send_text, not send_input, or a stray trailing Enter risks getting
  // consumed as input by whatever screen renders next (live-verified: see
  // commands.rs's send_text doc comment).
  function chooseOption(key: string) {
    setAction({ cmd: "send_text", args: { text: key } });
  }

  function sendFreeText(e: Event) {
    e.preventDefault();
    setAction({ cmd: "send_input", args: { text: freeText() } });
    setFreeText("");
  }

  function disconnect() {
    setAction({ cmd: "disconnect", args: {} });
  }

  function submitLogin(e: Event) {
    e.preventDefault();
    const values = loginValues();
    setAction({
      cmd: "login",
      args: {
        idNumber: values.id_number ?? "",
        accessCode: values.access_code ?? "",
        ssnLast4: values.ssn_last4 ?? "",
        birthDate: values.birth_date ?? "",
      },
    });
    setLoginValues({});
  }

  // The remote already closed the session server-side, so this just
  // overrides the resource's resolved value locally instead of firing a
  // command -- see `mutate`'s docs for why that's the right tool here.
  function reconnect() {
    mutate(null);
  }

  return (
    <main class="container">
      <Header busy={busy()} />
      <NoticeDialog dialog={dialog()} onClose={() => setDialog(null)} />

      <Show
        when={screen()}
        fallback={
          <ConnectForm
            username={username()}
            password={password()}
            onUsernameInput={setUsername}
            onPasswordInput={setPassword}
            onSubmit={(e) => {
              e.preventDefault();
              connect();
            }}
            busy={busy()}
          />
        }
      >
        <div class="screen">
          <Switch>
            <Match when={screen()?.kind === "MainMenu"}>
              <MainMenuScreen
                options={(screen() as Extract<TuiScreen, { kind: "MainMenu" }>).options}
                busy={busy()}
                onChoose={chooseOption}
              />
            </Match>

            <Match when={screen()?.kind === "Login"}>
              <LoginScreen
                fields={(screen() as Extract<TuiScreen, { kind: "Login" }>).fields}
                values={loginValues()}
                onChange={(key, value) => setLoginValues({ ...loginValues(), [key]: value })}
                onSubmit={submitLogin}
                busy={busy()}
              />
            </Match>

            <Match when={screen()?.kind === "SelectPeriod"}>
              <SelectPeriodScreen
                options={(screen() as Extract<TuiScreen, { kind: "SelectPeriod" }>).options}
                busy={busy()}
                onChoose={chooseOption}
              />
            </Match>

            <Match when={matricula()}>
              {(m) => (
                <MatriculaScreen
                  courses={m().courses}
                  prompt={m().prompt}
                  busy={busy()}
                  onChoose={chooseOption}
                  freeText={freeText()}
                  onFreeTextInput={setFreeText}
                  onFreeTextSubmit={sendFreeText}
                />
              )}
            </Match>

            <Match when={screen()?.kind === "Disconnected"}>
              <DisconnectedScreen busy={busy()} onReconnect={reconnect} />
            </Match>

            <Match when={screen()?.kind === "Unknown"}>
              <UnknownScreen
                raw={(screen() as Extract<TuiScreen, { kind: "Unknown" }>).raw}
                options={(screen() as Extract<TuiScreen, { kind: "Unknown" }>).options}
                busy={busy()}
                onChoose={chooseOption}
                freeText={freeText()}
                onFreeTextInput={setFreeText}
                onFreeTextSubmit={sendFreeText}
              />
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
