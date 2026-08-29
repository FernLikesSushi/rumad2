import type { Messages } from "./types";

const en: Messages = {
  title: "RUMAD 2 (Experimental)",
  loading: "Loading",
  close: "Close",
  loginHint:
    'SSH credentials for the RUMAD system. If left empty, defaults to the "estudiante" account. Estudiante account does not use a password.',
  usernamePlaceholder: "SSH Username",
  passwordPlaceholder: "SSH Password",
  connect: "Connect",
  connecting: "Connecting...",
  connectTagline1: "A friendly face to what you've already known.",
  connectTagline2: "A fresh coat of paint on a rusty old machine.",
  connectTagline3: "With a touch of modernity and a dash of nostalgia.",
  lokiModeBadge: "Loki Mode",
  lokiModeToast: "Loki mode enabled.",
  disconnectedTitle: "Session ended",
  disconnectedHint: "The remote system ended the session (PROCESO CONCLUIDO).",
  reconnect: "Reconnect",
  unknownHint: "Screen not recognized yet:",
  sendPlaceholder: "Send text...",
  login: "Log in",
  send: "Send",
  logout: "Log out",
  errorTitle: "Error",
  noticeTitle: "Notice",
  authTitle: "Authentication",
  developerMode: "Developer mode",
  settings: "Settings",
  devScreens: "Dev screens",
  navConnect: "Home",
  navClassEditor: "Class Editor",
  navMap: "Map",
  openMenu: "Open menu",
  closeMenu: "Close menu",
  madeWith: "Made with",
  footerBio:
    "By a software engineer who loves sushi, cake and exploding kittens.",
  continueLabel: "Continue",
  screenExit: "Leave this screen",
  weeklyScheduleTitle: "Class Schedule",
  weekdaysShort: ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat"],
  // Grounded in screens/matricula/select.txt's own footer ("Indique:
  // A=Alta B=Baja C=Cambio H=HorEst P=EvaluoPago M=MatEvaluo
  // F=HorEstGrafico O=CodigoReservar S=Salir") plus the matching MENU
  // DESPLIEGUE entries that HorEst/EvaluoPago/MatEvaluo/HorEstGrafico
  // abbreviate. Anything the remote adds later falls back to its raw
  // label rather than showing untranslated as blank.
  actionLabels: {
    Alta: "Add",
    Baja: "Drop",
    Cambio: "Change",
    HorEst: "Course Schedule",
    EvaluoPago: "Payment Estimate",
    MatEvaluo: "Enrollment Summary",
    HorEstGrafico: "Schedule Grid",
    CodigoReservar: "Reservation Code",
    Salir: "Exit",
  },
  periodLabels: {
    "1erVer": "1st Summer",
    "1erSem": "1st Semester",
    "2doSem": "2nd Semester",
    "2doVer": "2nd Summer",
  },
  // Grounded in the real transcripts `screens/mod.rs`'s
  // `classifies_real_main_menu_transcript` and `screens/menu.rs`'s
  // `classifies_menu_despliegue`/`classifies_select_period` tests check
  // against.
  menuLabels: {
    "***>>>  LEE tu Correo Electronico en ->  outlook.com":
      "***>>>  CHECK your Email at ->  outlook.com",
    "Seleccion de Secciones  (Matricula)": "Section Selection (Registration)",
    "Modificar Codigo de Acceso Permanente": "Change Permanent Access Code",
    "Informacion Correo Electronico": "Email Information",
    "Ver otra informacion": "View Other Information",
    "Seleccion de Modalidad P/D/F": "Select P/D/F Grading Mode",
    "SALIR DEL SISTEMA": "EXIT SYSTEM",
    "Evaluacion certificacion de Ayuda Economica":
      "Financial Aid Certification Evaluation",
    Curriculo: "Curriculum",
    "Evaluo de facturacion de matricula": "Tuition Billing Estimate",
    Matricula: "Registration",
    "Turno de seleccion de cursos/secciones o Examenes finales":
      "Course/Section Selection Time Slot or Final Exams",
    "Horario de cursos disponibles en Matricula":
      "Available Course Schedule in Registration",
    "Titulo de cursos disponibles en Horario": "Course Title List in Schedule",
    "Horario de matricula grafico": "Graphical Registration Schedule",
    "Evaluo de matricula e indicadores": "Registration Estimate and Indicators",
    Finalizar: "Finish",
    "1er Sem": "1st Semester",
    "2do Sem": "2nd Semester",
    "1er Verano o Verano Extendido": "1st Summer or Extended Summer",
    "2do Verano o Admision Temprana": "2nd Summer or Early Admission",
    salir: "exit",
  },
  loginFields: {
    idNumber: { label: "ID Number", hint: "e.g. 802999999" },
    accessCode: { label: "Permanent Access Code", hint: "e.g. 1234" },
    ssnLast4: { label: "Social Security (last 4)", hint: "e.g. 1234" },
    birthDate: { label: "Date of Birth", hint: "e.g. MMDDYYYY" },
  },
  loginPrivacyNotice:
    "This data is not stored or transmitted anywhere except to the official TUI backend, and is only used to log in and fetch your schedule.",
  menuTitles: {
    MainMenu: "MAIN MENU",
    MenuDespliegue: "OTHER INFORMATION",
    SelectPeriod: "Select Semester",
  },
  horarioMatriculaTitle: "Enrollment Schedule",
  searchHints: {
    HorarioCurso: "Course (e.g. QUIM3001L) - you may enter just the subject",
    HorarioSeccion: "Section (e.g. 001#)",
  },
  processingTitle: "Still processing...",
  matriculaTitle: "E N R O L L M E N T",
  matriculaColumns: {
    course: "Course",
    section: "Section",
    credits: "Cr.",
    status: "Grade",
  },
  courseResultsColumns: {
    section: "Sec.",
    room: "Room",
    schedule: "Schedule",
    credits: "Cr.",
    professor: "Professor",
    capacity: "Cap.",
    used: "Used",
    available: "Avail.",
  },
  weeklyScheduleColumns: { period: "Period" },
  confirmedScheduleTitle: "Confirmed Schedule",
  confirmedScheduleColumns: {
    course: "Course",
    section: "Section",
    credits: "Cr.",
    room: "Room",
    professor: "Professor",
  },
  newProfileDialog: {
    title: "New class profile",
    namePlaceholder: "Profile name",
    create: "Create",
    cancel: "Cancel",
    duplicateError: "A profile with that name already exists.",
  },
  classPreviewPicker: "Select a class",
  classPreviewView: { table: "Table", calendar: "Calendar" },
  classPreviewEmpty: "Select or create a profile to see its courses.",
  classProfileColumns: {
    course: "Course",
    section: "Section",
    room: "Room",
    schedule: "Schedule",
    credits: "Cr.",
    professor: "Professor",
  },
  addToClassProfile: "Add to class profile",
  addToClassProfileDisabledHint: "Select a class profile first",
  removeFromClassProfile: "Remove from class profile",
  courseAddedToast: "Course added to class profile.",
  courseRemovedToast: "Course removed from class profile.",
  openInMaps: { google: "Open in Google Maps", apple: "Open in Apple Maps", waze: "Open in Waze" },
};

export default en;
