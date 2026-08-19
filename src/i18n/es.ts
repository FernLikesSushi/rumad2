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
};

export default es;
