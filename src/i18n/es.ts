import type { Messages } from "./types";

// Default locale: the real remote system's own UI is Spanish, and that's
// this app's primary userbase. Only app-authored chrome lives here --
// text that comes back from the remote TUI (menu headings, notices,
// errors) is the remote's own output and is never translated.
const es: Messages = {
  title: "RUMAD",
  loading: "Cargando",
  close: "Cerrar",
  loginHint:
    'Deja el usuario y la clave en blanco para usar la cuenta compartida "estudiante" (solo menu principal). Ingresa tus propias credenciales de RUMAD para acceder a pantallas especificas de tu cuenta.',
  usernamePlaceholder: "Usuario / ID de estudiante",
  passwordPlaceholder: "Clave de acceso",
  connect: "Conectar",
  connecting: "Conectando...",
  disconnectedTitle: "Sesion finalizada",
  disconnectedHint: "El sistema remoto termino la sesion (PROCESO CONCLUIDO).",
  reconnect: "Reconectar",
  unknownHint: "Pantalla sin reconocer todavia:",
  sendPlaceholder: "Enviar texto...",
  send: "Enviar",
  logout: "Salir",
  errorTitle: "Error",
  noticeTitle: "Aviso",
  authTitle: "Autenticacion",
  developerMode: "Modo desarrollador",
  continueLabel: "Continuar",
  screenExit: "Salir de esta pantalla",
  weeklyScheduleTitle: "Horario de Matricula",
  // The remote's own labels are already Spanish, so there's nothing to
  // translate here -- the fallback-to-raw-label behavior covers it.
  actionLabels: {},
  loginFields: {
    idNumber: { label: "Número de Identificación", hint: "Ej. 802999999" },
    accessCode: { label: "Código de Acceso Permanente", hint: "Ej. 1234" },
    ssnLast4: { label: "Seguro Social (últimos 4)", hint: "Ej. 1234" },
    birthDate: { label: "Fecha de Nacimiento", hint: "Ej. MMDDAAAA" },
  },
  menuTitles: {
    MainMenu: "MENU PRINCIPAL",
    MenuDespliegue: "MENU DESPLIEGUE",
    SelectPeriod: "Indique Semestre",
  },
  horarioMatriculaTitle: "HORARIO DE MATRICULA",
  searchHints: {
    HorarioCurso: "Curso (Ej. QUIM3001L) - Puede indicar solo MATERIA",
    HorarioSeccion: "Seccion (Ej. 001#)",
  },
  processingTitle: "Programa en Proceso",
  matriculaTitle: "M A T R I C U L A",
  matriculaColumns: { course: "Curso", section: "Seccion", credits: "Cr.", status: "Grado" },
  courseResultsColumns: {
    section: "Sec.",
    room: "Salon",
    schedule: "Periodos",
    credits: "Crd.",
    professor: "Profesor",
    capacity: "Cap.",
    used: "Uti.",
    available: "Disp.",
  },
  weeklyScheduleColumns: { period: "Periodos" },
};

export default es;
