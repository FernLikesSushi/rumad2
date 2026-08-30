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
  // Connect screen's marketing taglines -- app chrome, not remote text.
  connectTagline1: string;
  connectTagline2: string;
  connectTagline3: string;
  // Easter egg (`Connect.tsx`, typing "loki") -- app chrome.
  lokiModeBadge: string;
  lokiModeToast: string;
  disconnectedTitle: string;
  disconnectedHint: string;
  reconnect: string;
  unknownHint: string;
  sendPlaceholder: string;
  send: string;
  login: string;
  logout: string;
  errorTitle: string;
  noticeTitle: string;
  authTitle: string;
  developerMode: string;
  settings: string;
  devScreens: string;
  // Side drawer nav (`Drawer.tsx`) -- app chrome, not remote text.
  navConnect: string;
  navClassEditor: string;
  navMap: string;
  openMenu: string;
  closeMenu: string;
  // The heart between "Made with"/"Hecho con" and "RUMAD" is a `Heart`
  // icon rendered in the footer's JSX, not part of this string -- kept
  // short so the icon reads as sitting inline between the two halves.
  madeWith: string;
  // App chrome, not remote text -- the whole "By a ..." sentence (unlike
  // `madeWith`, there's no icon interpolated into the middle of this one).
  footerBio: string;
  continueLabel: string;
  screenExit: string;
  weeklyScheduleTitle: string;
  // Short weekday labels for `WeekCalendar`, indexed by ISO 8601 weekday
  // (Monday=1) minus one -- [Mon, Tue, Wed, Thu, Fri, Sat]. No Sunday
  // entry since the remote's own schedules never include one.
  weekdaysShort: [string, string, string, string, string, string];
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
  // App-authored disclaimer shown under the login form -- not remote text,
  // so unlike the TUI's own labels it goes through `t()` like any other
  // app chrome.
  loginPrivacyNotice: string;
  // Screen titles and table column headers are hardcoded directly in
  // each screen component (like `Login`'s fields) rather than parsed off
  // the remote -- they're fixed, always-the-same structural labels for a
  // given screen kind, not per-user data, so hardcoding-and-localizing
  // them is the same tradeoff as `loginFields`, just per-screen instead
  // of per-field. Keyed by `MenuKind`/`SearchKind` (mirroring the Rust
  // structs those merge into, `MenuScreen`/`SearchScreen`) rather than
  // four/two separate named keys, so the catalog can't drift out of sync
  // with the merged type the way four independent keys could.
  menuTitles: {
    MainMenu: string;
    MenuDespliegue: string;
    SelectPeriod: string;
  };
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
  matriculaColumns: {
    course: string;
    section: string;
    credits: string;
    status: string;
  };
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
  confirmedScheduleTitle: string;
  confirmedScheduleColumns: {
    course: string;
    section: string;
    credits: string;
    room: string;
    professor: string;
  };
  // `ClassPreview`'s "+" button -- prompts for the name a new,
  // empty `ClassProfile` is saved under in `data/classEditor.ts`'s
  // persistent store.
  newProfileDialog: {
    title: string;
    namePlaceholder: string;
    create: string;
    cancel: string;
    duplicateError: string;
  };
  classPreviewPicker: string;
  // `ClassPreview`'s segmented control, switching its selected profile's
  // courses between `ClassProfileTable` and `WeekCalendar`.
  classPreviewView: { table: string; calendar: string };
  classPreviewEmpty: string;
  // `ClassProfileTable`'s columns -- `data/classEditor.ts`'s own `Course`
  // (user-authored profile entries), not the live-scraped `CourseSection`
  // `courseResultsColumns` covers, so no capacity/used/available here.
  classProfileColumns: {
    course: string;
    section: string;
    room: string;
    schedule: string;
    credits: string;
    professor: string;
  };
  // `CourseResultsTable`'s per-row "add to class profile" button --
  // `addToClassProfile` is the button's own aria-label, shown as the
  // tooltip too when enabled; `addToClassProfileDisabledHint` replaces it
  // while no profile is selected in `data/classEditor.ts`'s store.
  addToClassProfile: string;
  addToClassProfileDisabledHint: string;
  // `ClassProfileTable`'s remove-row button and `WeekCalendar`'s
  // per-event remove control (only rendered when a `CalendarEvent`
  // carries `onRemove`, which `ClassPreview`'s calendar view sets).
  removeFromClassProfile: string;
  // `data/classEditor.ts`'s `classEditorToast` -- shown globally
  // (see `ClassEditorToast.tsx`) after `addCourseToProfile`/
  // `removeCourseFromProfile` actually change something.
  courseAddedToast: string;
  courseRemovedToast: string;
  // `Map.tsx`'s "open externally" row -- mobile-only (`isMobile()`),
  // launches the platform's map app via `@tauri-apps/plugin-opener`'s
  // `openUrl` rather than anything rendered in-app.
  openInMaps: { google: string; apple: string; waze: string };
  // `Map.tsx`'s room code input -- typing a value and submitting navigates
  // to `/map/{roomCode}`, driving the same `useParams` the route itself does.
  roomCodePlaceholder: string;
  // `Map.tsx`'s room code table modal -- button label to open it and the
  // modal's own title. Each row is a link to `/map/{roomCode}`, same as
  // the input above.
  roomCodeListButton: string;
  roomCodeListTitle: string;
}
