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
    'Deja el usuario y la contraseña en blanco para usar la cuenta compartida "estudiante" (solo menu principal). Ingresa tus propias credenciales de RUMAD para acceder a pantallas especificas de tu cuenta.',
  usernamePlaceholder: "Usuario SSH",
  passwordPlaceholder: "Contraseña SSH",
  connect: "Conectar",
  connecting: "Conectando...",
  disconnectedTitle: "Sesion finalizada",
  disconnectedHint: "El sistema remoto termino la sesion (PROCESO CONCLUIDO).",
  reconnect: "Reconectar",
  unknownHint: "Pantalla sin reconocer todavia:",
  sendPlaceholder: "Enviar texto...",
  login: "Autenticar",
  send: "Enviar",
  logout: "Salir",
  errorTitle: "Error",
  noticeTitle: "Aviso",
  authTitle: "Autenticacion",
  developerMode: "Modo desarrollador",
  settings: "Configuracion",
  devScreens: "Pantallas de prueba",
  madeWith: "Hecho con",
  footerBio:
    "Por un ingeniero de software que ama el sushi, el bizcocho y los gatitos explosivos.",
  continueLabel: "Continuar",
  screenExit: "Salir de esta pantalla",
  weeklyScheduleTitle: "Horario de Matricula",
  weekdaysShort: ["Lun", "Mar", "Mie", "Jue", "Vie", "Sab"],
  // "Alta"/"Baja"/"Cambio"/"Salir" are already ordinary Spanish words, so
  // those fall back to the raw label -- but "HorEst"/"EvaluoPago"/
  // "MatEvaluo"/"HorEstGrafico"/"CodigoReservar" are squished CamelCase
  // abbreviations (see `screens/matricula/select.txt`), not readable on
  // their own, so those get expanded here too.
  actionLabels: {
    HorEst: "Horario Estudiantil",
    EvaluoPago: "Evaluo de Pago",
    MatEvaluo: "Matricula Evaluada",
    HorEstGrafico: "Horario Estudiantil Grafico",
    CodigoReservar: "Codigo de Reservacion",
  },
  // Unlike `actionLabels`, these raw labels are compact codes ("1erVer"),
  // not already-readable Spanish -- expanded to full words here.
  periodLabels: {
    "1erVer": "1er Verano",
    "1erSem": "1er Semestre",
    "2doSem": "2do Semestre",
    "2doVer": "2do Verano",
  },
  // MainMenu/MenuDespliegue/SelectPeriod's own option text is already
  // full, readable Spanish, so there's nothing to translate here -- the
  // fallback-to-raw-label behavior covers it.
  menuLabels: {},
  loginFields: {
    idNumber: { label: "Número de Identificación", hint: "Ej. 802999999" },
    accessCode: { label: "Código de Acceso Permanente", hint: "Ej. 1234" },
    ssnLast4: { label: "Seguro Social (últimos 4)", hint: "Ej. 1234" },
    birthDate: { label: "Fecha de Nacimiento", hint: "Ej. MMDDAAAA" },
  },
  loginPrivacyNotice:
    "Estos datos no se almacenan ni se transmiten a ningun otro lugar salvo el backend oficial del TUI, y solo se usan para autenticar y obtener tu horario.",
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
  matriculaColumns: {
    course: "Curso",
    section: "Seccion",
    credits: "Cr.",
    status: "Grado",
  },
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
  confirmedScheduleTitle: "Horario Confirmado",
  confirmedScheduleColumns: { course: "Curso", section: "Seccion", credits: "Cr.", room: "Salon", professor: "Profesor" },
};

export default es;
