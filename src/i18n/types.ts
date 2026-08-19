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
  // Same tradeoff as `actionLabels`, keyed by `HorarioSemester`'s own raw
  // compact labels ("1erVer", "1erSem", "2doSem", "2doVer" -- see
  // `menu.rs`'s `scrape_horario_semester_options`). Callers fall back to
  // the raw label the same way.
  periodLabels: Record<string, string>;
  // Same tradeoff again, keyed by `MainMenu`/`MenuDespliegue`/
  // `SelectPeriod`'s own raw option text (e.g. "Seleccion de Secciones
  // (Matricula)", "Curriculo", "1er Sem", "salir" -- see the real
  // transcripts in `screens/mod.rs`'s and `screens/menu.rs`'s tests).
  // Unlike `periodLabels`/`actionLabels`, this text is already
  // full/readable Spanish, not an abbreviation, so `es.ts` leaves it
  // empty and relies on the raw-label fallback.
  menuLabels: Record<string, string>;
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
  // of per-field. Keyed by `MenuKind`/`SearchKind` (mirroring the Rust
  // structs those merge into, `MenuScreen`/`SearchScreen`) rather than
  // four/two separate named keys, so the catalog can't drift out of sync
  // with the merged type the way four independent keys could.
  menuTitles: { MainMenu: string; MenuDespliegue: string; SelectPeriod: string };
  // `HorarioSemester` (one of the four `MenuKind`s) and both `SearchKind`s
  // share this one title -- the real remote header ("* HORARIO DE
  // MATRICULA *") is identical across all three, since they're three
  // steps of the same "Horario de cursos disponibles en Matricula" flow.
  horarioMatriculaTitle: string;
  // `SearchScreen` carries no data of its own (see its Rust doc comment)
  // -- its "(Ej. QUIM3001L)"/"(Ej. 001#)" hints are fixed boilerplate, not
  // per-user scraped content, so they're hardcoded and localized here
  // rather than threaded across the wire, same tradeoff as
  // `loginFields`/the screen titles above.
  searchHints: { HorarioCurso: string; HorarioSeccion: string };
  // `Dialog::Processing` (the remote's "Programa en Proceso" marquee)
  // carries no data either -- see that variant's Rust doc comment.
  processingTitle: string;
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
