export type MenuOption = { key: string; label: string };
export type LoginField = { key: string; label: string; hint: string };
export type ScheduleCourse = { slot: string; course: string; section: string; credits: string; status: string };
export type MatriculaPrompt =
  | { kind: "Actions"; options: MenuOption[] }
  | { kind: "Bajas" }
  | { kind: "Altas" }
  | { kind: "Cambio" };

export type TuiScreen =
  | { kind: "MainMenu"; options: MenuOption[] }
  | { kind: "Login"; fields: LoginField[] }
  | { kind: "SelectPeriod"; options: MenuOption[] }
  | { kind: "Matricula"; courses: ScheduleCourse[]; prompt: MatriculaPrompt }
  | { kind: "Notice"; message: string; raw: string }
  | { kind: "Disconnected" }
  | { kind: "Unknown"; raw: string; options: MenuOption[] };

export type Dialog = { title: string; message: string };

export type Action =
  | { cmd: "connect"; args: { username?: string; password?: string } }
  // Free-text prompts that expect a terminated line (course-code search,
  // Bajas/Altas/Cambio's "abbreviation or FIN").
  | { cmd: "send_input"; args: { text: string } }
  // Numbered/lettered menu options -- those screens read a single
  // keystroke with no Enter; see commands.rs's send_text doc comment for
  // the live-verified failure mode of appending one anyway.
  | { cmd: "send_text"; args: { text: string } }
  | { cmd: "login"; args: { idNumber: string; accessCode: string; ssnLast4: string; birthDate: string } }
  | { cmd: "disconnect"; args: {} };
