import { createSignal, For, Match, Switch } from "solid-js";
import { useNavigate } from "@solidjs/router";
import { t } from "../i18n";
import { devMode } from "../devMode";
import { mockScreens } from "../devScreens";
import type { TuiScreen, Send } from "../types";
import { Header } from "../components/Header";
import { MenuScreen } from "../screens/MenuScreen";
import { LoginScreen } from "../screens/Login";
import { MatriculaScreen } from "../screens/Matricula";
import { CourseResultsScreen } from "../screens/CourseResults";
import { WeeklyScheduleScreen } from "../screens/WeeklySchedule";
import { SearchScreen } from "../screens/SearchScreen";
import { DisconnectedScreen } from "../screens/Disconnected";
import { UnknownScreen } from "../screens/Unknown";

const KINDS: TuiScreen["kind"][] = [
  "Menu",
  "Login",
  "Matricula",
  "CourseResults",
  "WeeklySchedule",
  "Search",
  "Disconnected",
  "Unknown",
];

// Dev-only screen preview, gated on devMode() -- lets each screen component
// be checked visually against fabricated data without a live SSH session.
export function DevScreens() {
  const navigate = useNavigate();
  const [kind, setKind] = createSignal<TuiScreen["kind"]>("Menu");
  const send: Send = (action) => console.log("[dev] send", action);

  return (
    <>
      <Header />

      <Switch fallback={
        <div class="flex flex-col gap-4 items-center flex-1">
          <p class="text-[0.85em] opacity-75">Developer mode is off.</p>
          <button class="btn btn-outline btn-primary" onClick={() => navigate("/")}>
            {t().close}
          </button>
        </div>
      }>
        <Match when={devMode()}>
          <div class="flex flex-col gap-2.5 px-4 text-left flex-1">
            <div class="flex flex-wrap justify-center gap-2">
              <For each={KINDS}>
                {(k) => (
                  <button
                    class={`btn btn-sm ${k === kind() ? "btn-primary" : "btn-outline"}`}
                    onClick={() => setKind(k)}
                  >
                    {k}
                  </button>
                )}
              </For>
            </div>

            <div class="divider" />

            <Switch>
              <Match when={kind() === "Menu"}>
                <MenuScreen screen={mockScreens.Menu} busy={false} send={send} />
              </Match>
              <Match when={kind() === "Login"}>
                <LoginScreen login={() => console.log("[dev] login")} busy={false} />
              </Match>
              <Match when={kind() === "Matricula"}>
                <MatriculaScreen screen={mockScreens.Matricula} busy={false} send={send} />
              </Match>
              <Match when={kind() === "CourseResults"}>
                <CourseResultsScreen screen={mockScreens.CourseResults} busy={false} send={send} />
              </Match>
              <Match when={kind() === "WeeklySchedule"}>
                <WeeklyScheduleScreen screen={mockScreens.WeeklySchedule} busy={false} send={send} />
              </Match>
              <Match when={kind() === "Search"}>
                <SearchScreen screen={mockScreens.Search} busy={false} send={send} />
              </Match>
              <Match when={kind() === "Disconnected"}>
                <DisconnectedScreen busy={false} onReconnect={() => console.log("[dev] reconnect")} />
              </Match>
              <Match when={kind() === "Unknown"}>
                <UnknownScreen screen={mockScreens.Unknown} busy={false} send={send} />
              </Match>
            </Switch>
          </div>
        </Match>
      </Switch>
    </>
  );
}
