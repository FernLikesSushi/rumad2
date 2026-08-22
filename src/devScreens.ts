import type { TuiScreen } from "./types";

// Fabricated, not scraped -- realistic shapes for visually checking each
// screen component without a live SSH session.
export const mockScreens = {
  Menu: {
    kind: "Menu",
    menu: "MainMenu",
    options: [
      { key: "1", label: "Seleccion de Secciones  (Matricula)" },
      { key: "2", label: "Ver otra informacion" },
      { key: "3", label: "Curriculo" },
      { key: "0", label: "SALIR DEL SISTEMA" },
    ],
  },
  Matricula: {
    kind: "Matricula",
    courses: [
      { slot: "1", course: "MATE3031", section: "030", credits: "4", status: "Matriculado" },
      { slot: "2", course: "INGL3103", section: "045", credits: "3", status: "Matriculado" },
      { slot: "3", course: "HIST3220", section: "032", credits: "3", status: "Baja" },
    ],
    mode: {
      kind: "Actions",
      options: [
        { key: "A", label: "Alta" },
        { key: "B", label: "Baja" },
        { key: "C", label: "Cambio" },
        { key: "H", label: "HorEst" },
        { key: "S", label: "Salir" },
      ],
    },
  },
  CourseResults: {
    kind: "CourseResults",
    courseCode: "HIST 3220",
    courseTitle: "INT HIST:ENF SOCIAL",
    sections: [
      {
        section: "032",
        room: "CH316",
        schedule: "MJ       8:00- 9:15",
        credits: 3,
        professor: "J PEREZ RIVERA",
        capacity: 32,
        used: 32,
        available: 0,
        meetings: [
          { day: 2, startMinutes: 8 * 60, endMinutes: 9 * 60 + 15 },
          { day: 4, startMinutes: 8 * 60, endMinutes: 9 * 60 + 15 },
        ],
      },
      {
        section: "045",
        room: "CH120",
        schedule: "LWV     10:00-10:50",
        credits: 3,
        professor: "M TORRES DIAZ",
        capacity: 30,
        used: 25,
        available: 5,
        meetings: [
          { day: 1, startMinutes: 10 * 60, endMinutes: 10 * 60 + 50 },
          { day: 3, startMinutes: 10 * 60, endMinutes: 10 * 60 + 50 },
          { day: 5, startMinutes: 10 * 60, endMinutes: 10 * 60 + 50 },
        ],
      },
    ],
  },
  WeeklySchedule: {
    kind: "WeeklySchedule",
    days: ["Lunes", "Martes", "Miercoles", "Jueves", "Viernes", "Sabado"],
    rows: [
      { period: "8:30- 9:20", days: ["MATE3031 - 030", "", "MATE3031 - 030", "", "MATE3031 - 030", ""] },
      { period: "10:00-10:50", days: ["", "INGL3103 - 045", "", "INGL3103 - 045", "", ""] },
    ],
  },
  Search: { kind: "Search", search: "HorarioCurso" },
  Unknown: {
    kind: "Unknown",
    raw: "*** Pantalla no reconocida ***\n\n1) Opcion 1\n2) Opcion 2",
    options: [
      { key: "1", label: "Opcion 1" },
      { key: "2", label: "Opcion 2" },
    ],
  },
} satisfies Record<Exclude<TuiScreen["kind"], "Login" | "Disconnected">, TuiScreen>;
