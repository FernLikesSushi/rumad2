import { SettingsButton } from '../components/SettingsButton'
import { Match, Switch, Show, onMount } from "solid-js";
import { useNavigate } from "@solidjs/router";
import { t } from "../i18n";
import { devMode, setDevMode } from "../data/devMode";
import type { TuiScreen } from "../types";
import { ensureStarted, screen, busy, canExit, canContinue, send, exitScreen, continueScreen, login, disconnect as disconnectSession } from "../data/tui";
import { Spinner } from "../components/Spinner";
import { Toggle } from "../components/Toggle";
import { MenuScreen } from "../screens/MenuScreen";
import { LoginScreen } from "../screens/Login";
import { MatriculaScreen } from "../screens/Matricula";
import { CourseResultsScreen } from "../screens/CourseResults";
import { ConfirmedScheduleScreen } from "../screens/ConfirmedSchedule";
import { WeeklyScheduleScreen } from "../screens/WeeklySchedule";
import { SearchScreen } from "../screens/SearchScreen";
import { DisconnectedScreen } from "../screens/Disconnected";
import { UnknownScreen } from "../screens/Unknown";
import { ArrowLeft, LogOut, Settings, StepForward } from "lucide-solid";
import { Header } from '../components/Header';

// Renders whatever's in `data/tui.ts` and dispatches to the right screen
// component -- the session state itself lives there now (module-level, not
// owned by this component) so it survives navigating away from "/session"
// and back instead of being torn down and refetched.
export function TuiSession() {
  const navigate = useNavigate();

  onMount(() => ensureStarted());

  function goToConnect() {
    navigate("/", { replace: true });
  }

  async function disconnect() {
    await disconnectSession();
    goToConnect();
  }

  function reconnect() {
    goToConnect();
  }

  return (
    <>
      <Header disconnect={disconnect} />

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

              <Match when={screen().kind === "ConfirmedSchedule"}>
                <ConfirmedScheduleScreen
                  screen={screen() as Extract<TuiScreen, { kind: "ConfirmedSchedule" }>}
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
              <div class="flex flex-col justify-center gap-4 ">
                <Show when={canContinue()} keyed>
                  <button class="btn btn-outline btn-primary" disabled={busy()} onClick={continueScreen}>
                    <StepForward />
                    {t().continueLabel}
                  </button>
                </Show>

                <Show when={canExit()} keyed>
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
