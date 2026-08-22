export type MenuOption = { key: string; label: string };
export type LoginField = { key: string; label: string; hint: string };
export type ScheduleCourse = { slot: string; course: string; section: string; credits: string; status: string };
export type MatriculaMode =
  | { kind: "Actions"; options: MenuOption[] }
  | { kind: "Bajas" }
  | { kind: "Altas" }
  | { kind: "Cambio" };

export type Meeting = { day: number; startMinutes: number; endMinutes: number };

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

export type ScheduleRow = { period: string; days: string[] };

// Mirrors Rust's `MenuKind`/`SearchKind` 
export type MenuKind = "MainMenu" | "MenuDespliegue" | "SelectPeriod" | "HorarioSemester";
export type SearchKind = "HorarioCurso" | "HorarioSeccion";

export type TuiScreen =
  | { kind: "Menu"; menu: MenuKind; options: MenuOption[] }
  | { kind: "Login"; fields: LoginField[] }
  | { kind: "Matricula"; courses: ScheduleCourse[]; mode: MatriculaMode }
  | { kind: "CourseResults"; courseCode: string; courseTitle: string; sections: CourseSection[] }
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

// UI state for the notice dialog component -- derived from a `Dialog`
// (specifically `Notice`; `Processing` doesn't open this), not the same
// thing as one.
export type DialogBox = { title: string; message: string };

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

// Mirrors `commands::interact::send` (the backend's single generic entry
// point for `SendAction`) one-for-one: App.tsx owns the actual dispatch
// (there's only one `createResource`/`action` signal to drive), but each
// screen component gets just this raw primitive and builds its own
// specific functions from it -- e.g. a `choose(key)` calling
// `send({kind:"Select",key})` -- exactly like each Rust screen struct's
// own `RumadScreen` impl decides what its `select`/`line`/`exit` methods
// actually do, rather than `commands/interact.rs` deciding that for them.
export type Send = (action: SendAction) => void;

// Shared prop shape for every screen component that renders a specific
// `TuiScreen` variant (everything `TuiRouter`'s `Switch`/`Match` dispatches
// to except `LoginScreen`/`DisconnectedScreen`, whose kinds carry no data)
// -- `screen` is narrowed to just that variant via `T`, so the component
// destructures its own fields off `props.screen` instead of `TuiRouter`
// unpacking them into separate named props.
export type TuiScreenComponentProps<T extends TuiScreen["kind"]> = {
  screen: Extract<TuiScreen, { kind: T }>;
  busy: boolean;
  send: Send;
};

export type Action =
  | { cmd: "connect"; args: { username?: string; password?: string } }
  | { cmd: "send"; args: { action: SendAction } }
  | { cmd: "login"; args: { idNumber: string; accessCode: string; ssnLast4: string; birthDate: string } }
  | { cmd: "get_screen"; args: {} }
  | { cmd: "disconnect"; args: {} };
