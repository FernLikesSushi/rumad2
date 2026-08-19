export type MenuOption = { key: string; label: string };
export type LoginField = { key: string; label: string; hint: string };
export type ScheduleCourse = { slot: string; course: string; section: string; credits: string; status: string };
export type MatriculaPrompt =
  | { kind: "Actions"; options: MenuOption[] }
  | { kind: "Bajas" }
  | { kind: "Altas" }
  | { kind: "Cambio" };

export type CourseSection = {
  section: string;
  room: string;
  schedule: string;
  credits: string;
  professor: string;
  capacity: string;
  used: string;
  available: string;
};

export type ScheduleRow = { period: string; days: string[] };

export type TuiScreen =
  | { kind: "MainMenu"; options: MenuOption[] }
  | { kind: "Login"; fields: LoginField[] }
  | { kind: "SelectPeriod"; options: MenuOption[] }
  | { kind: "Matricula"; courses: ScheduleCourse[]; prompt: MatriculaPrompt }
  | { kind: "CourseResults"; courseCode: string; courseTitle: string; sections: CourseSection[] }
  | { kind: "WeeklySchedule"; days: string[]; rows: ScheduleRow[] }
  | { kind: "Notice"; message: string; raw: string }
  | { kind: "Disconnected" }
  | { kind: "Unknown"; raw: string; options: MenuOption[] };

export type Dialog = { title: string; message: string };

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
  | { kind: "Exit" };

// Mirrors `commands::interact::send` (the backend's single generic entry
// point for `SendAction`) one-for-one: App.tsx owns the actual dispatch
// (there's only one `createResource`/`action` signal to drive), but each
// screen component gets just this raw primitive and builds its own
// specific functions from it -- e.g. a `choose(key)` calling
// `send({kind:"Select",key})` -- exactly like each Rust screen struct's
// own `RumadScreen` impl decides what its `select`/`line`/`exit` methods
// actually do, rather than `commands/interact.rs` deciding that for them.
export type Send = (action: SendAction) => void;

export type Action =
  | { cmd: "connect"; args: { username?: string; password?: string } }
  | { cmd: "send"; args: { action: SendAction } }
  | { cmd: "login"; args: { idNumber: string; accessCode: string; ssnLast4: string; birthDate: string } }
  | { cmd: "disconnect"; args: {} };
