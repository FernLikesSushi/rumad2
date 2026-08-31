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
    'Deja el usuario y la contraseña en blanco para usar la cuenta compartida "estudiante" (solo menú principal). Ingresa tus propias credenciales de RUMAD para acceder a pantallas específicas de tu cuenta.',
  usernamePlaceholder: "Usuario SSH",
  passwordPlaceholder: "Contraseña SSH",
  connect: "Conectar",
  connecting: "Conectando...",
  // Not translations of the English taglines -- independent lines in the
  // same spirit (playful, warm, about making RUMAD less painful to use).
  connectTagline1: "El RUMAD de siempre, ahora sin los dolores de cabeza.",
  connectTagline2: "Coger clases no debería ser doloroso.",
  connectTagline3: "Hecho con cariño boricua, para que matricules tranquilo.",
  lokiModeBadge: "Modo Loki",
  lokiModeToast: "Modo Loki activado.",
  disconnectedTitle: "Sesión finalizada",
  disconnectedHint: "El sistema remoto terminó la sesión (PROCESO CONCLUIDO).",
  reconnect: "Reconectar",
  unknownHint: "Pantalla sin reconocer todavía:",
  sendPlaceholder: "Enviar texto...",
  login: "Autenticar",
  send: "Enviar",
  logout: "Salir",
  errorTitle: "Error",
  noticeTitle: "Aviso",
  authTitle: "Autenticación",
  developerMode: "Modo desarrollador",
  settings: "Configuración",
  devScreens: "Pantallas de prueba",
  navConnect: "Inicio",
  navClassEditor: "Perfiles de Clases",
  navMap: "Mapa",
  openMenu: "Abrir menú",
  closeMenu: "Cerrar menú",
  madeWith: "Hecho con",
  footerBio:
    "Por un ingeniero de software que ama el sushi, el bizcocho y los gatitos explosivos.",
  continueLabel: "Continuar",
  screenExit: "Salir de esta pantalla",
  weeklyScheduleTitle: "Horario de Matrícula",
  weekdaysShort: ["Lun", "Mar", "Mié", "Jue", "Vie", "Sáb"],
  // "Alta"/"Baja"/"Cambio"/"Salir" are already ordinary Spanish words, so
  // those fall back to the raw label -- but "HorEst"/"EvaluoPago"/
  // "MatEvaluo"/"HorEstGrafico"/"CodigoReservar" are squished CamelCase
  // abbreviations (see `screens/matricula/select.txt`), not readable on
  // their own, so those get expanded here too.
  actionLabels: {
    HorEst: "Horario Estudiantil",
    EvaluoPago: "Evalúo de Pago",
    MatEvaluo: "Matrícula Evaluada",
    HorEstGrafico: "Horario Estudiantil Gráfico",
    CodigoReservar: "Código de Reservación",
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
    idNumber: { label: "Número de Identificación", hint: "Ej. 802-99-9999" },
    accessCode: { label: "Código de Acceso Permanente", hint: "Ej. 1234" },
    ssnLast4: { label: "Seguro Social (últimos 4)", hint: "Ej. 1234" },
    birthDate: { label: "Fecha de Nacimiento", hint: "Ej. MMDDAAAA" },
  },
  loginPrivacyNotice:
    "Estos datos no se almacenan ni se transmiten a ningún otro lugar salvo el backend oficial del TUI, y solo se usan para autenticar y obtener tu horario.",
  menuTitles: {
    MainMenu: "MENU PRINCIPAL",
    MenuDespliegue: "MENU DESPLIEGUE",
    SelectPeriod: "Indique Semestre",
  },
  horarioMatriculaTitle: "HORARIO DE MATRICULA",
  searchHints: {
    HorarioCurso: "Curso (Ej. QUIM3001L) - Puede indicar solo MATERIA",
    HorarioSeccion: "Sección (Ej. 001#)",
  },
  processingTitle: "Programa en Proceso",
  matriculaTitle: "M A T R I C U L A",
  matriculaColumns: {
    course: "Curso",
    section: "Sección",
    credits: "Cr.",
    status: "Grado",
  },
  courseResultsColumns: {
    section: "Sec.",
    room: "Salón",
    schedule: "Períodos",
    credits: "Crd.",
    professor: "Profesor",
    capacity: "Cap.",
    used: "Uti.",
    available: "Disp.",
  },
  weeklyScheduleColumns: { period: "Períodos" },
  confirmedScheduleTitle: "Horario Confirmado",
  confirmedScheduleColumns: {
    course: "Curso",
    section: "Sección",
    credits: "Cr.",
    room: "Salón",
    professor: "Profesor",
  },
  newProfileDialog: {
    title: "Nuevo perfil de clases",
    namePlaceholder: "Nombre del perfil",
    create: "Crear",
    cancel: "Cancelar",
    duplicateError: "Ya existe un perfil con ese nombre.",
  },
  classPreviewPicker: "Selecciona una clase",
  classPreviewView: { table: "Tabla", calendar: "Calendario" },
  classPreviewEmpty: "Selecciona o crea un perfil para ver sus cursos.",
  classProfileColumns: {
    course: "Curso",
    section: "Sección",
    room: "Salón",
    schedule: "Períodos",
    credits: "Cr.",
    professor: "Profesor",
  },
  addToClassProfile: "Añadir al perfil de clases",
  addToClassProfileDisabledHint: "Selecciona un perfil de clases primero",
  removeFromClassProfile: "Quitar del perfil de clases",
  courseAddedToast: "Curso añadido al perfil de clases.",
  courseRemovedToast: "Curso removido del perfil de clases.",
  openInMaps: { google: "Abrir en Google Maps", apple: "Abrir en Apple Maps", waze: "Abrir en Waze" },
  roomCodePlaceholder: "Codigo de salon (ej. S200)",
  roomCodeListButton: "Ver todos los edificios",
  roomCodeListTitle: "Codigos de edificios",
};

export default es;
