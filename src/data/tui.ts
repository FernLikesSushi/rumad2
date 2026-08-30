import { createMemo, createResource, createRoot, createSignal } from "solid-js";
import { listen } from "@tauri-apps/api/event";
import { runAction } from "../screens/api";
import { haptic } from "../haptics";

// Wire-format types below: these mirror the Rust backend's serialized
// shapes one-to-one (`src-tauri/src/screens/`, `src-tauri/src/commands/`).

export type MenuOption = { key: string; label: string };
export type LoginField = { key: string; label: string; hint: string };
// One row of the student's schedule on the `Matricula` screen (the
// interactive course list itself -- no room/schedule/professor data,
// see `CourseSection`/`ConfirmedCourse` below for those).
export type ScheduleCourse = { slot: string; course: string; section: string; credits: string; status: string };
export type MatriculaMode =
  | { kind: "Actions"; options: MenuOption[] }
  | { kind: "Bajas" }
  | { kind: "Altas" }
  | { kind: "Cambio" };

export type Meeting = { day: number; startMinutes: number; endMinutes: number };

// One open section of a course, from `CourseResults` (`MENU DESPLIEGUE`'s
// "Horario de cursos disponibles en Matricula" search): a read-only
// lookup across every section of one course code, distinct from the
// student's own schedule (`ScheduleCourse`/`ConfirmedCourse`).
export type CourseSection = {
  section: string;
  room: string;
  schedule: string;
  credits: number;
  professor: string;
  capacity: number;
  used: number;
  available: number;
  meetings: Meeting[];
};

// One enrolled course from `ConfirmedSchedule` -- `Matricula`'s own
// `[CONFIRMADA]` report shown after actually confirming a schedule, with
// section/credits/room/meeting data `ScheduleCourse` doesn't carry.
export type ConfirmedCourse = {
  course: string;
  section: string;
  credits: number;
  room: string;
  professor: string;
  meetings: Meeting[];
};

export type ScheduleRow = { period: string; days: string[] };

// Mirrors Rust's `MenuKind`/`SearchKind`
export type MenuKind = "MainMenu" | "MenuDespliegue" | "SelectPeriod" | "HorarioSemester";
export type SearchKind = "HorarioCurso" | "HorarioSeccion";

export type TuiScreen =
  | { kind: "Menu"; menu: MenuKind; options: MenuOption[] }
  | { kind: "Login"; fields: LoginField[] }
  | { kind: "Matricula"; courses: ScheduleCourse[]; mode: MatriculaMode }
  | { kind: "CourseResults"; courseCode: string; courseTitle: string; sections: CourseSection[] }
  | { kind: "ConfirmedSchedule"; courses: ConfirmedCourse[] }
  | { kind: "WeeklySchedule"; days: string[]; rows: ScheduleRow[] }
  | { kind: "Search"; search: SearchKind }
  | { kind: "Disconnected" }
  | { kind: "Unknown"; raw: string; options: MenuOption[] };

// Mirrors Rust's `Dialog` -- a message shown *alongside* whatever
// `TuiScreen` is currently rendering, never a screen replacement itself.
// See `ClassifiedScreen`.
export type Dialog = { kind: "Notice"; message: string; raw: string } | { kind: "Processing" };

// Mirrors Rust's `ClassifiedScreen`: every command/event resolves to one
// of these, always a fresh `TuiScreen` plus an optional `Dialog` overlaid
// on top of it -- App.tsx renders `screen` directly (no more "keep
// showing the previous screen while a Notice is up" reducer trick the old
// design needed) and derives the notice/processing UI from `dialog`
// alongside it. `canExit` mirrors `RumadScreen::can_exit` -- whether the
// one shared exit control should render, instead of each screen component
// deciding that for itself.
export type ClassifiedScreen = { screen: TuiScreen; dialog: Dialog | null; canExit: boolean; canContinue: boolean };

// Mirrors Rust's `SendAction` (`commands/interact.rs`) -- the backend
// re-classifies the current screen and dispatches through its own
// `RumadScreen` impl, so which of these is valid depends on what's
// currently showing.
export type SendAction =
  // Numbered/lettered menu options -- those screens read a single
  // keystroke with no Enter; see `RumadScreen`'s doc comment for the
  // live-verified failure mode of appending one anyway.
  | { kind: "Select"; key: string }
  // Free-text prompts that expect a terminated line (course-code search,
  // Bajas/Altas/Cambio's "abbreviation or FIN").
  | { kind: "Line"; text: string }
  // A dedicated back/leave keystroke that isn't one of the screen's
  // listed options (e.g. Login's PF4).
  | { kind: "Exit" }
  // "Enter to continue" on a read-only screen (CourseResults, WeeklySchedule).
  | { kind: "Continue" };

// Mirrors the Tauri commands in `src-tauri/src/commands/` one-for-one --
// the `cmd`/`args` shape `invoke()` expects. See `screens/api.ts`'s
// `runAction`, the only caller.
export type Action =
  | { cmd: "connect"; args: { username?: string; password?: string } }
  | { cmd: "send"; args: { action: SendAction } }
  | { cmd: "login"; args: { idNumber: string; accessCode: string; ssnLast4: string; birthDate: string } }
  | { cmd: "get_screen"; args: {} }
  | { cmd: "disconnect"; args: {} };

// Mirrors `commands::interact::send` (the backend's single generic entry
// point for `SendAction`) one-for-one: `TuiSession.tsx` owns the actual
// dispatch (there's only one `createResource`/`action` signal to drive,
// below), but each screen component gets just this raw primitive and
// builds its own specific functions from it -- e.g. a `choose(key)`
// calling `send({kind:"Select",key})` -- exactly like each Rust screen
// struct's own `RumadScreen` impl decides what its `select`/`line`/`exit`
// methods actually do, rather than `commands/interact.rs` deciding that
// for them.
export type Send = (action: SendAction) => void;

// Module-level (not owned by any component) so the current screen/busy
// state survives navigating away from "/session" and back (e.g. to
// "/map") instead of being torn down and refetched -- `TuiSession.tsx`
// renders whatever's here rather than owning it itself. The real source of
// truth is still the Rust backend's `AppState`; this is just a persistent
// cache of its last response, kept in sync the same way it always was: the
// `action`/`response` resource, plus the `screen-changed` event for
// redraws the backend pushes on its own. Dialog-showing is a separate
// concern -- `TuiDialogHandler` reacts to `response` below rather than
// this module owning a dialog signal itself.

// `false`, not an initial `get_screen` action -- this module is imported
// (transitively, via `Drawer.tsx`) before anyone has connected at all, and
// `false` is `createResource`'s built-in way to say "don't fetch yet".
// `ensureStarted` kicks off the real first fetch once `TuiSession.tsx`
// actually mounts (post-connect); a `false` source doubles as "no live
// session" for `connected` below.
const [action, setAction] = createSignal<Action | false>(false);

export function ensureStarted() {
  if (action() === false) setAction({ cmd: "get_screen", args: {} });
}

export function connect(username?: string, password?: string) {
  haptic();
  setAction({ cmd: "connect", args: { username: username || undefined, password: password || undefined } });
}

// Only the resource/memos need an owning root -- Solid's documented
// pattern for a permanent, app-lifetime store (as opposed to one scoped
// to a component), so it doesn't warn "computations created outside a
// createRoot... will never be disposed": never disposing is the point
// here, not an oversight.
export const { response, screen, busy, canExit, canContinue, connected, mutate } = createRoot(() => {
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

  // Whether a TUI session is live -- derived straight from `screen`, not a
  // separate `is_connected` poll: `screen` is already kept in sync with
  // the real backend state via `screen-changed` (Rust pushes that the
  // moment it confirms the SSH channel actually closed, see
  // `handle_scene_change`/`Disconnected` in commands/exec.rs), so re-asking
  // over IPC would just be checking the same truth a second, slower way.
  const connected = createMemo(() => screen() !== undefined && screen()?.kind !== "Disconnected");

  return { response, screen, busy, canExit, canContinue, connected, mutate };
});

// Backend polls in the background for screens that redraw a second time on
// their own (`Dialog::Processing`) and pushes this event when done, and
// also emits it directly from the `disconnect` command -- one event either
// way a session ends, so this is the single place that reacts to that.
// Attached once, for the app's lifetime, rather than per-mount -- there's
// no route-scoped onCleanup to unlisten from anymore.
listen<ClassifiedScreen>("screen-changed", (event) => {
  mutate(event.payload);
  // Reset back to "no live session" -- otherwise `action` would keep
  // pointing at whatever was last dispatched against the now-dropped
  // session, and `ensureStarted`'s `action() === false` check would never
  // see a reason to fetch again the next time "/session" mounts.
  if (event.payload.screen.kind === "Disconnected") setAction(false);
});

// quick helper to avoid repeating the `setAction({cmd:"send",args:{action}})` boilerplate everywhere
export const send: Send = (sendAction) => {
  haptic();
  setAction({ cmd: "send", args: { action: sendAction } });
};

export function exitScreen() {
  send({ kind: "Exit" });
}

export function continueScreen() {
  send({ kind: "Continue" });
}

export function login(idNumber: string, accessCode: string, ssnLast4: string, birthDate: string) {
  haptic();
  setAction({ cmd: "login", args: { idNumber, accessCode, ssnLast4, birthDate } });
}

// Just the backend call -- navigating back to "/" afterward is the
// caller's job (`TuiSession.tsx` has the router context this doesn't).
export async function disconnect() {
  haptic();
  await runAction({ cmd: "disconnect", args: {} });
}
