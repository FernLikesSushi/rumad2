import type { Messages } from "./types";

const en: Messages = {
  title: "RUMAD",
  loading: "Loading",
  close: "Close",
  loginHint:
    'Leave username and password blank to use the shared "estudiante" demo account (main menu only). Enter your own RUMAD credentials to reach account-specific screens.',
  usernamePlaceholder: "Username / Student ID",
  passwordPlaceholder: "Password",
  connect: "Connect",
  connecting: "Connecting...",
  disconnectedTitle: "Session ended",
  disconnectedHint: "The remote system ended the session (PROCESO CONCLUIDO).",
  reconnect: "Reconnect",
  unknownHint: "Screen not recognized yet:",
  sendPlaceholder: "Send text...",
  send: "Send",
  logout: "Log out",
  errorTitle: "Error",
  noticeTitle: "Notice",
  authTitle: "Authentication",
  developerMode: "Developer mode",
  continueLabel: "Continue",
  screenExit: "Leave this screen",
  weeklyScheduleTitle: "Class Schedule",
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
  loginFields: {
    idNumber: { label: "ID Number", hint: "e.g. 802999999" },
    accessCode: { label: "Permanent Access Code", hint: "e.g. 1234" },
    ssnLast4: { label: "Social Security (last 4)", hint: "e.g. 1234" },
    birthDate: { label: "Date of Birth", hint: "e.g. MMDDYYYY" },
  },
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
  matriculaColumns: { course: "Course", section: "Section", credits: "Cr.", status: "Grade" },
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
};

export default en;
