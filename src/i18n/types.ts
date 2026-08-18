// Shared shape for every locale catalog -- a missing or extra key in any
// es.ts/en.ts is a compile error, so the catalogs can't drift apart.
export interface Messages {
  title: string;
  loading: string;
  close: string;
  loginHint: string;
  usernamePlaceholder: string;
  passwordPlaceholder: string;
  connect: string;
  connecting: string;
  disconnectedTitle: string;
  disconnectedHint: string;
  reconnect: string;
  unknownHint: string;
  sendPlaceholder: string;
  send: string;
  logout: string;
  errorTitle: string;
  noticeTitle: string;
}
