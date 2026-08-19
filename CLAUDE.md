# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

A Tauri v2 + SolidJS desktop app that drives an interactive SSH session against the real UPRM student system (`rumad.uprm.edu`, an OpenVMS "SISTEMA ESTUDIANTIL COLEGIAL" VT100 TUI) and exposes it to a web frontend as typed screens instead of raw terminal output.

## Commands

Use `pnpm`, not `npm`, for all JS/TS dependency management.

```bash
pnpm install                 # install frontend deps
pnpm dev                     # vite dev server only (port 1420)

cd src-tauri
cargo build                  # build the Rust backend
cargo test                   # run backend tests
cargo test <test_name>       # run a single test, e.g. cargo test classifies_login_form
npx tsc --noEmit -p tsconfig.json   # frontend typecheck (from repo root)
npx vite build                      # frontend production build (from repo root)
```

There is no `cargo tauri dev`/`cargo tauri build` wired up cleanly yet: `tauri.conf.json`'s `beforeDevCommand` is `"deno task dev"` but no `deno.json` exists in the repo, so the Tauri CLI's dev/build flow may not work as-is. Use the VS Code launch config instead (`.vscode/launch.json`, F5 → "Tauri Development Debug"): it starts `pnpm dev` itself and launches the compiled binary directly, bypassing that setting.

## Architecture

**Backend pipeline** (`src-tauri/src/`): `ssh/session.rs`'s `TuiSession` holds one long-lived interactive SSH channel plus a `vt100::Parser` virtual screen (connect once, then `send_line`/`send_text`/`send_key` per user action — no scripted run-to-completion). `screens/mod.rs`'s `classify(raw) -> TuiScreen` turns the current raw screen text into a typed, serializable enum the frontend renders as native UI instead of a terminal emulator. `commands/` exposes this as Tauri commands (`connect`, `get_screen`, `send`, `login`, `disconnect`) backed by `Mutex<Option<TuiSession>>` in managed state -- `mod.rs` holds just `AppState` and the simple `#[tauri::command]` handlers, `exec.rs` has the shared lock-act-classify runner (`act`, taking a plain `FnOnce(&mut TuiSession) -> anyhow::Result<()>` closure) plus logging, `interact.rs` is the `send` command specifically (see below), and `login.rs` is the `login` command specifically -- it re-reads and classifies the *current* screen, confirms it's actually `TuiScreen::Login`, and only then calls that screen's own `LoginScreen::login(session, ...)` (see below). `login`/`interact` are `pub mod`, not re-exported, because `#[tauri::command]` expands to hidden sibling items `generate_handler!` needs to find at the function's real path.

**`TuiScreen`'s variants each wrap a named struct** (`MainMenuScreen`, `LoginScreen`, `SelectPeriodScreen`, `MatriculaScreen`, `NoticeScreen`, `UnknownScreen`; `Disconnected` has no data) rather than holding fields directly -- this is what lets `RumadScreen`/`LoginScreen` be real `impl` targets. Serde's internal tagging (`#[serde(tag = "kind")]`) flattens a newtype variant wrapping a struct exactly the same as a struct variant would, so the JSON shape the frontend sees is unchanged.

**`screens/interact.rs`'s `RumadScreen` trait** is the single point every screen interaction goes through -- default `select`/`line`/`exit` methods for "pick a numbered/lettered option (no Enter)" / "submit a terminated line of free text" / "go back". It lives in `screens/`, not `commands/`, because it's defined in terms of the screen structs `screens/` already owns; `TuiScreen::as_rumad_screen()` (in `screens/mod.rs`, alongside `TuiScreen`'s other methods) maps a classified screen to its impl, returning `None` for the two kinds that aren't directly interactive (`Notice`, `Disconnected`). `commands/interact.rs`'s `send` command is the only caller: it re-classifies the *current* screen (same pattern as `login.rs`) and dispatches an incoming `SendAction` (`Select{key}` / `Line{text}` / `Exit`) through whichever trait method matches. `LoginScreen` implements the trait only for `exit()` — entry deviates too much from "pick an option" (fixed-width fields, no Enter, whitespace-sensitive) to share `select`/`line`'s defaults, so it gets its own `login()` method for that and overrides both to reject calls outright; `exit()` sends PF4 (confirmed live from the screen's own footer, "PF4=(9)") instead of the "0" every other screen's default sends. `MatriculaScreen` overrides `exit()` to reject too -- its `Actions` prompt has no "0" option (`S`=Salir is a normal `select`) and its `Bajas`/`Altas`/`Cambio` sub-prompts exit via the free-text line "FIN" instead, so there's no single grounded exit keystroke to default to.

**Every command is `async fn` + `spawn_blocking`, never a plain sync fn that does I/O.** Verified directly from the `tauri`/`tauri-macros` source: a plain (non-`async fn`) `#[tauri::command]` runs *inline* on whatever thread dispatches the IPC message — the GTK main loop on Linux — with no automatic thread-pool dispatch. A blocking SSH call there freezes the entire window. Commands take `AppHandle` (not `State<AppState>`, which can't move into a `'static` closure) and fetch `app.state::<AppState>()` from inside the `spawn_blocking` closure.

**`screens/` is one submodule per screen type** (`login.rs`, `select_period.rs`, `matricula.rs`, `notice.rs`, plus `scrape.rs` for cross-screen scraping helpers and `interact.rs` for the cross-screen `RumadScreen` trait), each owning its own types and tests. The one thing that stays in `mod.rs` rather than moving into a submodule is `classify()` itself — see below for why.

**Screen classification is a priority-ordered chain**, not independent pattern matches — order matters and is deliberate, which is why `classify()` lives as one function in `screens/mod.rs` instead of being split across the per-screen submodules:
1. `Disconnected` — the remote's own "PROCESO CONCLUIDO" graceful-session-end banner (checked first; `commands/exec.rs`'s `finish` drops the session from state when seen).
2. The one specific `Notice` that means "action failed" (`"...no esta disponible..."`) — checked before specific screen types because a rejected action redisplays the previous screen *with the rejection notice still on it*; matching the screen type first would silently swallow the notice. `TuiScreen::or_err()` promotes this one to a command `Err`.
3. Specific, grounded screen types (`Login`, `SelectPeriod`, `Matricula`, `MainMenu`).
4. A generalized `<< message >>` advisory/notice pattern (`notice::extract_bracketed_notice`) — low priority, checked last before the `Unknown` fallback, so it never overrides an already-modeled screen. Unlike #2 this isn't an action-failure signal, so it must not win over genuinely useful structured content.
5. `Unknown` — raw text plus best-effort scraped numbered/lettered options, so unmodeled screens stay navigable rather than dead ends.

**`Notice` is a UI concept, not a screen.** The frontend treats it as a dialog overlaid on whatever's currently showing (via a `createMemo` with a previous-value reducer that returns `prev` for `Notice` results) rather than a screen replacement. Any new screen kind that's "a message alongside the current screen" should follow this same pattern instead of getting its own render branch.

**Frontend** (`src/`) is built around Solid's actual async primitives, not hand-rolled state: `App.tsx` is the orchestrator only (state signals, the `createResource`, the memos, the `Switch`/`Match` dispatch) — each `TuiScreen` variant renders through its own component in `src/screens/` (`MainMenu.tsx`, `Login.tsx`, `SelectPeriod.tsx`, `Matricula.tsx`, `Disconnected.tsx`, `Unknown.tsx`, plus `ConnectForm.tsx` for the pre-connect fallback), with cross-screen UI (`OptionButtons`, `FreeTextForm`, `NoticeDialog`, `Header`) factored into `src/components/`. Shared types live in `src/types.ts`, and the `runAction` dispatcher lives in `src/api.ts`. Every user action calls `setAction({cmd, args})` (a fresh object each time) feeding `createResource(action, runAction)`. `response.loading` is the busy state, `response.error` drives the error dialog, and "what screen to render" is a `createMemo`, not an effect writing to a signal — deriving render state belongs in a memo, effects are for genuine side effects (like opening the dialog). `mutate()` (the resource's second return value) is used to override the resolved screen locally without firing a real command, e.g. "Reconectar" after a `Disconnected` screen.

**i18n** (`src/i18n/`): per-locale catalogs (`es.ts` default, `en.ts`) typed against a shared `Messages` interface so a missing/extra key is a compile error. Only app-authored UI chrome goes through `t()` — text mirroring the remote TUI (menu headings, notices, error messages) is the remote's own Spanish output and is never translated. Two deliberate exceptions, both because the underlying text is hardcoded rather than parsed off the live remote screen (so there's a fixed string to translate in the first place, unlike genuinely scraped content): `Messages.actionLabels`, an open-ended `Record<string, string>` (not a named key, since it's not fixed app chrome) keyed by `Matricula`'s `Actions` prompt's own raw labels ("HorEst", "CodigoReservar", ...) — `es.ts` leaves it `{}` since the remote's Spanish needs no translation, `en.ts` populates it grounded in `screens/matricula/select.txt`'s footer, and callers (`Matricula.tsx`'s `localizedActions`) must fall back to the raw label for anything not listed, since this one can't be exhaustive against remote text the way named `Messages` keys are; and `Messages.loginFields`, a named (not `Record`) key since `Login`'s 4 fields are a fixed, known set — `Login.tsx` hardcodes its field list from `t().loginFields` instead of the backend's `TuiScreen::Login.fields` (mangled over the wire regardless, same reason `LoginField`'s Rust-side doc comment hardcodes it there), so both `es.ts` and `en.ts` carry real translations, no fallback needed.

## Screen transcripts (`screens/`)

New remote screens get added here as real transcripts before any corresponding `TuiScreen` variant is written — each screen submodule's tests `include_str!` these files directly rather than retyping their content, so a test fails loudly if the real formatting ever changes. This directory is a moving target: it can be reorganized into subdirectories (e.g. `screens/matricula/`) and gains new files across sessions — always re-list it rather than trusting a remembered layout.

**Redaction rule:** every screen dropped in here must have personally identifying and institution-specific data redacted and replaced with reasonable, randomized stand-ins before being committed — student ID numbers, names, course codes/sections, professor names, meeting times, and similar fields. Keep the *shape* of the data realistic (right number of digits, plausible course-code format, etc.) since parsers are built against these files; just don't leave real people's or real courses' data in the repo.

**Don't build a `TuiScreen` variant speculatively.** Every existing variant was written against a real (redacted) transcript in this directory, not guessed remote prompt text. If a screen in here doesn't have a corresponding typed variant yet, that's expected — it renders via the `Unknown` fallback until someone asks for it to be modeled.
