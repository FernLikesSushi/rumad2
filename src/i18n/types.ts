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
  authTitle: string;
  developerMode: string;
  continueLabel: string;
  screenExit: string;
  weeklyScheduleTitle: string;
  // Keyed by the remote's own raw label text (e.g. "HorEst",
  // "CodigoReservar") for options this app localizes despite them coming
  // from the remote (Matricula's Actions prompt) -- an open-ended lookup,
  // not part of the app's own fixed chrome, so it stays a Record instead
  // of named keys. Callers must fall back to the raw label for anything
  // not listed here (a screen the remote adds later, an option this
  // catalog hasn't been updated for yet, ...).
  actionLabels: Record<string, string>;
  // `Login`'s 4 fields are hardcoded on both ends (see `Login.tsx`'s doc
  // comment -- the remote's own labels for this specific form arrive
  // mangled over the wire) rather than read off the backend's `fields`,
  // so unlike other remote-mirroring text, these are safe to localize: a
  // fixed, known set, not open-ended scraped content.
  loginFields: {
    idNumber: { label: string; hint: string };
    accessCode: { label: string; hint: string };
    ssnLast4: { label: string; hint: string };
    birthDate: { label: string; hint: string };
  };
  // Screen titles and table column headers are hardcoded directly in
  // each screen component (like `Login`'s fields) rather than parsed off
  // the remote -- they're fixed, always-the-same structural labels for a
  // given screen kind, not per-user data, so hardcoding-and-localizing
  // them is the same tradeoff as `loginFields`, just per-screen instead
  // of per-field.
  mainMenuTitle: string;
  menuDespliegueTitle: string;
  horarioMatriculaTitle: string;
  // `HorarioCursoScreen` carries no data of its own (see its Rust doc
  // comment) -- its "(Ej. QUIM3001L) Puede indicar solo MATERIA" hint is
  // fixed boilerplate, not per-user scraped content, so it's hardcoded and
  // localized here rather than threaded across the wire, same tradeoff as
  // `loginFields`/the screen titles above.
  horarioCursoHint: string;
  // Same tradeoff as `horarioCursoHint`, for `HorarioSeccionScreen`'s
  // "(Ej. 001#)" hint.
  horarioSeccionHint: string;
  // `TuiScreen::Processing` (the remote's "Programa en Proceso" marquee)
  // carries no data either -- see that variant's Rust doc comment.
  processingTitle: string;
  selectPeriodTitle: string;
  matriculaTitle: string;
  matriculaColumns: { course: string; section: string; credits: string; status: string };
  courseResultsColumns: {
    section: string;
    room: string;
    schedule: string;
    credits: string;
    professor: string;
    capacity: string;
    used: string;
    available: string;
  };
  weeklyScheduleColumns: { period: string };
}
