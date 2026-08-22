import { SettingsButton } from './../components/SettingsButton'
import { createSignal, createResource, createMemo, createEffect, onMount, onCleanup, Match, Switch, Show } from "solid-js";
import { useNavigate } from "@solidjs/router";
import { listen } from "@tauri-apps/api/event";
import { t } from "../i18n";
import { runAction } from "../api";
import { devMode, setDevMode } from "../devMode";
import type { TuiScreen, ClassifiedScreen, Action, Send } from "../types";
import { NoticeDialog, createDialog } from "../components/NoticeDialog";
import { Spinner } from "../components/Spinner";
import { Toggle } from "../components/Toggle";
import { MenuScreen } from "../screens/MenuScreen";
import { LoginScreen } from "../screens/Login";
import { MatriculaScreen } from "../screens/Matricula";
import { CourseResultsScreen } from "../screens/CourseResults";
import { WeeklyScheduleScreen } from "../screens/WeeklySchedule";
import { SearchScreen } from "../screens/SearchScreen";
import { DisconnectedScreen } from "../screens/Disconnected";
import { UnknownScreen } from "../screens/Unknown";
import { ArrowLeft, LogOut, Settings, StepForward } from "lucide-solid";
import { Header } from '../components/Header';

// Owns the TUI session's state once connected -- the send/login/disconnect
// resource, the notice dialog, the busy spinner, and dispatch to the
// right screen component.
export function TuiRouter() {
  const navigate = useNavigate();
  const { dialog: dialogBox, show: showDialog, close: closeDialog } = createDialog();

  const [action, setAction] = createSignal<Action>({ cmd: "get_screen", args: {} });
  const [response, { mutate }] = createResource(action, runAction);

  // `Dialog::Processing` counts as busy too. Must check response.error
  // before response() -- a resolved rejection otherwise throws uncaught
  // inside this memo.
  const busy = createMemo(() => {
    if (response.loading) return true;
    if (response.error) return false;
    return response()?.dialog?.kind === "Processing";
  });

  // Same response.error-before-response() rule as `busy` -- a rejected
  // command (e.g. a restricted MainMenu option) throws here otherwise.
  const screen = createMemo<TuiScreen | undefined>((prev) => {
    if (response.loading || response.error) return prev;
    return response()?.screen ?? prev;
  });

  const canExit = createMemo<boolean>((prev) => {
    if (response.loading || response.error) return prev;
    return response()?.canExit ?? prev;
  }, false);

  const canContinue = createMemo<boolean>((prev) => {
    if (response.loading || response.error) return prev;
    return response()?.canContinue ?? prev;
  }, false);

  createEffect(() => {
    if (response.loading) return;
    const err = response.error;
    if (err) {
      showDialog({ title: t().errorTitle, message: String(err) });
      return;
    }
    const result = response();
    if (result?.dialog?.kind === "Notice") {
      showDialog({ title: t().noticeTitle, message: result.dialog.message });
    }
  });

  const send: Send = (sendAction) => setAction({ cmd: "send", args: { action: sendAction } });

  function exitScreen() {
    send({ kind: "Exit" });
  }

  function continueScreen() {
    send({ kind: "Continue" });
  }

  function login(idNumber: string, accessCode: string, ssnLast4: string, birthDate: string) {
    setAction({ cmd: "login", args: { idNumber, accessCode, ssnLast4, birthDate } });
  }

  function goToConnect() {
    navigate("/", { replace: true });
  }

  async function disconnect() {
    await runAction({ cmd: "disconnect", args: {} });
    goToConnect();
  }

  function reconnect() {
    goToConnect();
  }

  // Backend polls in the background for screens that redraw a second time
  // on their own (`Dialog::Processing`) and pushes this event when done.
  onMount(() => {
    const unlisten = listen<ClassifiedScreen>("screen-changed", (event) => mutate(event.payload));
    onCleanup(() => void unlisten.then((f) => f()));
  });

  return (
    <>
      <Header disconnect={disconnect} />

      <NoticeDialog dialog={dialogBox()} onClose={closeDialog} />

      <Show when={screen()}>
        {(screen) => (
          <div class="flex flex-col gap-2.5 px-4 text-left flex-1">
            <Switch>
              <Match when={screen().kind === "Menu"}>
                <MenuScreen screen={screen() as Extract<TuiScreen, { kind: "Menu" }>} busy={busy()} send={send} />
              </Match>

              <Match when={screen().kind === "Login"}>
                <LoginScreen login={login} busy={busy()} />
              </Match>

              <Match when={screen().kind === "Matricula"}>
                <MatriculaScreen
                  screen={screen() as Extract<TuiScreen, { kind: "Matricula" }>}
                  busy={busy()}
                  send={send}
                />
              </Match>

              <Match when={screen().kind === "CourseResults"}>
                <CourseResultsScreen
                  screen={screen() as Extract<TuiScreen, { kind: "CourseResults" }>}
                  busy={busy()}
                  send={send}
                />
              </Match>

              <Match when={screen().kind === "WeeklySchedule"}>
                <WeeklyScheduleScreen
                  screen={screen() as Extract<TuiScreen, { kind: "WeeklySchedule" }>}
                  busy={busy()}
                  send={send}
                />
              </Match>

              <Match when={screen().kind === "Search"}>
                <SearchScreen screen={screen() as Extract<TuiScreen, { kind: "Search" }>} busy={busy()} send={send} />
              </Match>

              <Match when={screen().kind === "Disconnected"}>
                <DisconnectedScreen busy={busy()} onReconnect={reconnect} />
              </Match>

              <Match when={screen().kind === "Unknown"}>
                <UnknownScreen
                  screen={screen() as Extract<TuiScreen, { kind: "Unknown" }>}
                  busy={busy()}
                  send={send}
                />
              </Match>
            </Switch>

            <div class="flex flex-col items-center gap-2.5 py-16">
              <div class="grid grid-cols-2 grid-rows-1 justify-center gap-4 ">
                <Show when={canContinue()} fallback={<div />} keyed>
                  <button class="btn btn-outline btn-primary" disabled={busy()} onClick={continueScreen}>
                    <StepForward />
                    {t().continueLabel}
                  </button>
                </Show>

                <Show when={canExit()} fallback={<div />} keyed>
                  <button class="btn items-center" disabled={busy()} onClick={exitScreen}>
                    <ArrowLeft />
                    {t().screenExit}
                  </button>
                </Show>
              </div>
            </div>
          </div>
        )}
      </Show>

      <Show when={busy()}>
        <div class="mt-4 text-center">
          <Spinner />
        </div>
      </Show>
    </>
  );
}
