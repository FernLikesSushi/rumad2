import { createSignal, createResource, createMemo, createEffect, onMount, onCleanup, Match, Switch, Show } from "solid-js";
import { listen } from "@tauri-apps/api/event";
import { t } from "./i18n";
import { runAction } from "./api";
import type { TuiScreen, ClassifiedScreen, DialogBox, Action, Send } from "./types";
import { NoticeDialog } from "./components/NoticeDialog";
import { Spinner } from "./components/Spinner";
import { MenuScreen } from "./screens/MenuScreen";
import { LoginScreen } from "./screens/Login";
import { MatriculaScreen } from "./screens/Matricula";
import { CourseResultsScreen } from "./screens/CourseResults";
import { WeeklyScheduleScreen } from "./screens/WeeklySchedule";
import { SearchScreen } from "./screens/SearchScreen";
import { DisconnectedScreen } from "./screens/Disconnected";
import { UnknownScreen } from "./screens/Unknown";

// Everything TUI-state-related once a connection exists: `Connect` owns
// just the initial `connect` call and hands the resulting `initial`
// screen here, then never runs again -- this owns the send/login/
// disconnect resource (seeded with `initial` via `createResource`'s own
// `initialValue`, so there's no gap before the first real action),
// the notice dialog, the busy/processing spinner, and dispatching to the
// right screen component. The only way back out is `onDisconnected`,
// fired once the session is genuinely gone -- explicit logout, or the
// remote's own "PROCESO CONCLUIDO" (`Disconnected`'s "Reconectar")-- at
// which point App.tsx unmounts this and remounts `Connect`.
export function TuiRouter(props: { initial: ClassifiedScreen; onDisconnected: () => void }) {
  const [dialogBox, setDialogBox] = createSignal<DialogBox | null>(null);

  const [action, setAction] = createSignal<Action>();
  const [response, { mutate }] = createResource(action, runAction, { initialValue: props.initial });

  // A `Dialog::Processing` overlay counts as busy too -- the remote is
  // still computing, so every screen's own controls stay disabled the
  // same way they do while a command is genuinely in flight.
  //
  // Same "check `response.error` before reading `response()`" hazard as
  // `screen`/`canExit` below (see their shared comment) -- this one is
  // easy to miss since `response.loading` short-circuits the common case,
  // but a resolved *rejection* still needs its own check before the `||`
  // falls through to reading `response()`, or this throws uncaught and
  // silently breaks rendering (live-confirmed: the error dialog for
  // "Opcion NO esta disponible por el momento" never appeared because of
  // exactly this).
  const busy = createMemo(() => {
    if (response.loading) return true;
    if (response.error) return false;
    return response()?.dialog?.kind === "Processing";
  });

  // What to render is derived, not imperatively assigned. `disconnect()`/
  // `reconnect()` below never route the "session is gone" case back
  // through this resource (unlike the old design) -- they call
  // `props.onDisconnected()` directly instead -- so `response()` (once
  // resolved) is always a real `ClassifiedScreen` here, never null.
  //
  // `response.error` must be checked *before* reading `response()` --
  // Solid's resources throw the rejection reason when you read a resource
  // that errored (that's how `<ErrorBoundary>` integration works), and
  // this memo has no boundary around it. A command can genuinely reject
  // (e.g. the backend's `ClassifiedScreen::or_err()` promoting a rejection
  // notice to an `Err`, live-confirmed: selecting a demo-account-restricted
  // MainMenu option returns "Opcion NO esta disponible por el momento" as
  // an error, not a screen) -- reading `response()` unconditionally in
  // that case throws inside the memo with nothing to catch it, which
  // stalls this computation (and anything downstream) instead of just
  // showing the error dialog the other effect already handles.
  const screen = createMemo<TuiScreen>((prev) => {
    if (response.loading || response.error) return prev;
    return response()?.screen ?? prev;
  }, props.initial.screen);

  // Whether the one shared exit control (below) should render -- mirrors
  // `screen` itself: derived straight off the backend's own
  // `ClassifiedScreen.canExit` rather than each screen component deciding
  // for itself.
  const canExit = createMemo<boolean>((prev) => {
    if (response.loading || response.error) return prev;
    return response()?.canExit ?? prev;
  }, props.initial.canExit);

  // `Matricula`'s mode is nested one level deeper than the other screen
  // kinds, which makes repeated inline `as Extract<...>` casts unwieldy --
  // narrow it once here instead.
  const matricula = createMemo(() => {
    const s = screen();
    return s.kind === "Matricula" ? s : undefined;
  });

  // Opening the notice dialog is a genuine side effect (and independently
  // dismissible via the close button), unlike `screen`/`canExit` above --
  // it belongs in an effect, not a memo. `Dialog::Processing` doesn't open
  // this -- it's reflected in `busy` instead, since it's transient and
  // resolves on its own via the `screen-changed` listener below rather
  // than needing an explicit dismissal.
  createEffect(() => {
    if (response.loading) return;
    const err = response.error;
    if (err) {
      setDialogBox({ title: t().errorTitle, message: String(err) });
      return;
    }
    const result = response();
    if (result?.dialog?.kind === "Notice") {
      setDialogBox({ title: t().noticeTitle, message: result.dialog.message });
    }
  });

  const send: Send = (sendAction) => setAction({ cmd: "send", args: { action: sendAction } });

  function exitScreen() {
    send({ kind: "Exit" });
  }

  function login(idNumber: string, accessCode: string, ssnLast4: string, birthDate: string) {
    setAction({ cmd: "login", args: { idNumber, accessCode, ssnLast4, birthDate } });
  }

  // Ends the session -- bypasses `action`/`response` entirely rather than
  // dispatching through the shared resource: there's nothing left for it
  // to reflect once the session is gone, so this is a plain async call
  // followed by handing back to `Connect` via `onDisconnected`.
  async function disconnect() {
    await runAction({ cmd: "disconnect", args: {} });
    props.onDisconnected();
  }

  // The remote already closed the session server-side (`Disconnected`
  // screen's own "Reconectar" button) -- no backend call needed, just
  // hand back to `Connect` the same way `disconnect` above does.
  function reconnect() {
    props.onDisconnected();
  }

  // The backend's `spawn_screen_watcher` (`commands/exec.rs`) keeps
  // polling in the background for screens that redraw a second time on
  // their own (`Dialog::Processing`) and pushes this event once that
  // happens, instead of the frontend needing to poll for it.
  onMount(() => {
    const unlisten = listen<ClassifiedScreen>("screen-changed", (event) => mutate(event.payload));
    onCleanup(() => void unlisten.then((f) => f()));
  });

  return (
    <>
      <NoticeDialog dialog={dialogBox()} onClose={() => setDialogBox(null)} />

      <div class="screen">
        <Switch>
          <Match when={screen().kind === "Menu"}>
            {(() => {
              const s = () => screen() as Extract<TuiScreen, { kind: "Menu" }>;
              return <MenuScreen menu={s().menu} options={s().options} busy={busy()} send={send} />;
            })()}
          </Match>

          <Match when={screen().kind === "Login"}>
            <LoginScreen login={login} busy={busy()} />
          </Match>

          <Match when={matricula()}>
            {(m) => <MatriculaScreen courses={m().courses} mode={m().mode} busy={busy()} send={send} />}
          </Match>

          <Match when={screen().kind === "CourseResults"}>
            <CourseResultsScreen
              courseCode={(screen() as Extract<TuiScreen, { kind: "CourseResults" }>).courseCode}
              courseTitle={(screen() as Extract<TuiScreen, { kind: "CourseResults" }>).courseTitle}
              sections={(screen() as Extract<TuiScreen, { kind: "CourseResults" }>).sections}
              busy={busy()}
              send={send}
            />
          </Match>

          <Match when={screen().kind === "WeeklySchedule"}>
            <WeeklyScheduleScreen
              days={(screen() as Extract<TuiScreen, { kind: "WeeklySchedule" }>).days}
              rows={(screen() as Extract<TuiScreen, { kind: "WeeklySchedule" }>).rows}
              busy={busy()}
              send={send}
            />
          </Match>

          <Match when={screen().kind === "Search"}>
            <SearchScreen
              search={(screen() as Extract<TuiScreen, { kind: "Search" }>).search}
              busy={busy()}
              send={send}
            />
          </Match>

          <Match when={screen().kind === "Disconnected"}>
            <DisconnectedScreen busy={busy()} onReconnect={reconnect} />
          </Match>

          <Match when={screen().kind === "Unknown"}>
            <UnknownScreen
              raw={(screen() as Extract<TuiScreen, { kind: "Unknown" }>).raw}
              options={(screen() as Extract<TuiScreen, { kind: "Unknown" }>).options}
              busy={busy()}
              send={send}
            />
          </Match>
        </Switch>

        <div class="row">
          <Show when={canExit()}>
            <button disabled={busy()} onClick={exitScreen}>
              {t().screenExit}
            </button>
          </Show>
          <Show when={screen().kind !== "Disconnected"}>
            <button class="disconnect" disabled={busy()} onClick={disconnect}>
              {t().logout}
            </button>
          </Show>
        </div>
      </div>

      <Show when={busy()}>
        <div class="footer text-5xl">
          <Spinner />
        </div>
      </Show>
    </>
  );
}
