import { createSignal, createResource, createMemo, createEffect, onMount, onCleanup, Match, Switch, Show } from "solid-js";
import { listen } from "@tauri-apps/api/event";
import { t } from "./i18n";
import { runAction } from "./api";
import type { TuiScreen, Dialog, Action, Send } from "./types";
import { Header } from "./components/Header";
import { NoticeDialog } from "./components/NoticeDialog";
import { ConnectForm } from "./screens/ConnectForm";
import { MainMenuScreen } from "./screens/MainMenu";
import { LoginScreen } from "./screens/Login";
import { SelectPeriodScreen } from "./screens/SelectPeriod";
import { MatriculaScreen } from "./screens/Matricula";
import { CourseResultsScreen } from "./screens/CourseResults";
import { WeeklyScheduleScreen } from "./screens/WeeklySchedule";
import { MenuDespliegueScreen } from "./screens/MenuDespliegue";
import { HorarioSemesterScreen } from "./screens/HorarioSemester";
import { HorarioCursoScreen } from "./screens/HorarioCurso";
import { HorarioSeccionScreen } from "./screens/HorarioSeccion";
import { ProcessingScreen } from "./screens/Processing";
import { DisconnectedScreen } from "./screens/Disconnected";
import { UnknownScreen } from "./screens/Unknown";
import "./App.css";

// The orchestrator: owns the one `action` signal/`createResource` this app
// is built around (there can only be one -- Solid resources aren't
// per-component) and derives what's currently showing from it. Everything
// screen-specific -- what to call a given `SendAction`, what local form
// state a screen needs -- lives in that screen's own component instead of
// being decided here; this just hands each one the raw dispatch
// primitives (`send`, `login`, `connect`) it needs. Mirrors the backend:
// `commands/interact.rs::send` is one generic entry point, but what
// `Select`/`Line`/`Exit` actually mean for a given screen is decided by
// that screen's own `RumadScreen` impl, not by the command itself.
function App() {
  const [dialog, setDialog] = createSignal<Dialog | null>(null);

  const [action, setAction] = createSignal<Action>();
  const [response, { mutate }] = createResource(action, runAction);
  const busy = () => response.loading;

  // What to render is derived, not imperatively assigned: `Notice` is a
  // message alongside the current screen rather than a screen change, so
  // the memo keeps whatever was showing instead of switching to it.
  //
  // `response.error` must be checked *before* reading `response()` --
  // Solid's resources throw the rejection reason when you read a resource
  // that errored (that's how `<ErrorBoundary>` integration works), and
  // this memo has no boundary around it. A command can genuinely reject
  // (e.g. the backend's `TuiScreen::or_err()` promoting a rejection notice
  // to an `Err`, live-confirmed: selecting a demo-account-restricted
  // MainMenu option returns "Opcion NO esta disponible por el momento" as
  // an error, not a screen) -- reading `response()` unconditionally in
  // that case throws inside the memo with nothing to catch it, which
  // stalls this computation (and anything downstream) instead of just
  // showing the error dialog the other effect already handles.
  const screen = createMemo<TuiScreen | null>((prev) => {
    if (response.loading || response.error) return prev ?? null;
    const result = response();
    if (result === undefined || result?.kind === "Notice") return prev ?? null;
    return result;
  }, null);

  // `Matricula`'s mode is nested one level deeper than the other screen
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

  function connect(username: string, password: string) {
    setAction({ cmd: "connect", args: { username: username || undefined, password: password || undefined } });
  }

  const send: Send = (sendAction) => setAction({ cmd: "send", args: { action: sendAction } });

  function login(idNumber: string, accessCode: string, ssnLast4: string, birthDate: string) {
    setAction({ cmd: "login", args: { idNumber, accessCode, ssnLast4, birthDate } });
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

  // The backend's `spawn_screen_watcher` (`commands/exec.rs`) keeps
  // polling in the background for screens that redraw a second time on
  // their own (`TuiScreen::Processing`) and pushes this event once that
  // happens, instead of the frontend needing to poll for it -- same
  // `mutate` override `reconnect` above uses, just driven by the backend
  // rather than a user action.
  onMount(() => {
    const unlisten = listen<TuiScreen>("screen-changed", (event) => mutate(event.payload));
    onCleanup(() => void unlisten.then((f) => f()));
  });

  return (
    <main class="container">
      <Header busy={busy()} />
      <NoticeDialog dialog={dialog()} onClose={() => setDialog(null)} />

      <Show when={screen()} fallback={<ConnectForm onConnect={connect} busy={busy()} />}>
        <div class="screen">
          <Switch>
            <Match when={screen()?.kind === "MainMenu"}>
              <MainMenuScreen
                options={(screen() as Extract<TuiScreen, { kind: "MainMenu" }>).options}
                busy={busy()}
                send={send}
              />
            </Match>

            <Match when={screen()?.kind === "Login"}>
              <LoginScreen login={login} send={send} busy={busy()} />
            </Match>

            <Match when={screen()?.kind === "SelectPeriod"}>
              <SelectPeriodScreen
                options={(screen() as Extract<TuiScreen, { kind: "SelectPeriod" }>).options}
                busy={busy()}
                send={send}
              />
            </Match>

            <Match when={matricula()}>
              {(m) => <MatriculaScreen courses={m().courses} mode={m().mode} busy={busy()} send={send} />}
            </Match>

            <Match when={screen()?.kind === "CourseResults"}>
              <CourseResultsScreen
                courseCode={(screen() as Extract<TuiScreen, { kind: "CourseResults" }>).courseCode}
                courseTitle={(screen() as Extract<TuiScreen, { kind: "CourseResults" }>).courseTitle}
                sections={(screen() as Extract<TuiScreen, { kind: "CourseResults" }>).sections}
                busy={busy()}
                send={send}
              />
            </Match>

            <Match when={screen()?.kind === "WeeklySchedule"}>
              <WeeklyScheduleScreen
                days={(screen() as Extract<TuiScreen, { kind: "WeeklySchedule" }>).days}
                rows={(screen() as Extract<TuiScreen, { kind: "WeeklySchedule" }>).rows}
                busy={busy()}
                send={send}
              />
            </Match>

            <Match when={screen()?.kind === "MenuDespliegue"}>
              <MenuDespliegueScreen
                options={(screen() as Extract<TuiScreen, { kind: "MenuDespliegue" }>).options}
                busy={busy()}
                send={send}
              />
            </Match>

            <Match when={screen()?.kind === "HorarioSemester"}>
              <HorarioSemesterScreen
                options={(screen() as Extract<TuiScreen, { kind: "HorarioSemester" }>).options}
                busy={busy()}
                send={send}
              />
            </Match>

            <Match when={screen()?.kind === "HorarioCurso"}>
              <HorarioCursoScreen busy={busy()} send={send} />
            </Match>

            <Match when={screen()?.kind === "HorarioSeccion"}>
              <HorarioSeccionScreen busy={busy()} send={send} />
            </Match>

            <Match when={screen()?.kind === "Processing"}>
              <ProcessingScreen />
            </Match>

            <Match when={screen()?.kind === "Disconnected"}>
              <DisconnectedScreen busy={busy()} onReconnect={reconnect} />
            </Match>

            <Match when={screen()?.kind === "Unknown"}>
              <UnknownScreen
                raw={(screen() as Extract<TuiScreen, { kind: "Unknown" }>).raw}
                options={(screen() as Extract<TuiScreen, { kind: "Unknown" }>).options}
                busy={busy()}
                send={send}
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
