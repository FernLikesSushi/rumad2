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

**Backend pipeline** (`src-tauri/src/`): `ssh/session.rs`'s `TuiSession` holds one long-lived interactive SSH channel plus a `vt100::Parser` virtual screen (connect once, then `send_line`/`send_text`/`send_key` per user action — no scripted run-to-completion). `screens.rs`'s `classify(raw) -> TuiScreen` turns the current raw screen text into a typed, serializable enum the frontend renders as native UI instead of a terminal emulator. `commands.rs` exposes this as Tauri commands (`connect`, `get_screen`, `send_input`, `send_text`, `send_key`, `disconnect`) backed by `Mutex<Option<TuiSession>>` in managed state.

**Every command is `async fn` + `spawn_blocking`, never a plain sync fn that does I/O.** Verified directly from the `tauri`/`tauri-macros` source: a plain (non-`async fn`) `#[tauri::command]` runs *inline* on whatever thread dispatches the IPC message — the GTK main loop on Linux — with no automatic thread-pool dispatch. A blocking SSH call there freezes the entire window. Commands take `AppHandle` (not `State<AppState>`, which can't move into a `'static` closure) and fetch `app.state::<AppState>()` from inside the `spawn_blocking` closure.

**Screen classification is a priority-ordered chain**, not independent pattern matches — order matters and is deliberate:
1. `Disconnected` — the remote's own "PROCESO CONCLUIDO" graceful-session-end banner (checked first; `commands::finish` drops the session from state when seen).
2. The one specific `Notice` that means "action failed" (`"...no esta disponible..."`) — checked before specific screen types because a rejected action redisplays the previous screen *with the rejection notice still on it*; matching the screen type first would silently swallow the notice. `TuiScreen::or_err()` promotes this one to a command `Err`.
3. Specific, grounded screen types (`Login`, `SelectPeriod`, `Matricula`, `MainMenu`).
4. A generalized `<< message >>` advisory/notice pattern (`extract_bracketed_notice`) — low priority, checked last before the `Unknown` fallback, so it never overrides an already-modeled screen. Unlike #2 this isn't an action-failure signal, so it must not win over genuinely useful structured content.
5. `Unknown` — raw text plus best-effort scraped numbered/lettered options, so unmodeled screens stay navigable rather than dead ends.

**`Notice` is a UI concept, not a screen.** The frontend treats it as a dialog overlaid on whatever's currently showing (via a `createMemo` with a previous-value reducer that returns `prev` for `Notice` results) rather than a screen replacement. Any new screen kind that's "a message alongside the current screen" should follow this same pattern instead of getting its own render branch.

**Frontend** (`src/App.tsx`) is built around Solid's actual async primitives, not hand-rolled state: every user action calls `setAction({cmd, args})` (a fresh object each time) feeding `createResource(action, runAction)`. `response.loading` is the busy state, `response.error` drives the error dialog, and "what screen to render" is a `createMemo`, not an effect writing to a signal — deriving render state belongs in a memo, effects are for genuine side effects (like opening the dialog). `mutate()` (the resource's second return value) is used to override the resolved screen locally without firing a real command, e.g. "Reconectar" after a `Disconnected` screen.

**i18n** (`src/i18n/`): per-locale catalogs (`es.ts` default, `en.ts`) typed against a shared `Messages` interface so a missing/extra key is a compile error. Only app-authored UI chrome goes through `t()` — text mirroring the remote TUI (menu headings, notices, error messages, form field labels) is the remote's own Spanish output and is never translated.

## Screen transcripts (`screens/`)

New remote screens get added here as real transcripts before any corresponding `TuiScreen` variant is written — `screens.rs`'s tests `include_str!` these files directly rather than retyping their content, so a test fails loudly if the real formatting ever changes. This directory is a moving target: it can be reorganized into subdirectories (e.g. `screens/matricula/`) and gains new files across sessions — always re-list it rather than trusting a remembered layout.

**Redaction rule:** every screen dropped in here must have personally identifying and institution-specific data redacted and replaced with reasonable, randomized stand-ins before being committed — student ID numbers, names, course codes/sections, professor names, meeting times, and similar fields. Keep the *shape* of the data realistic (right number of digits, plausible course-code format, etc.) since parsers are built against these files; just don't leave real people's or real courses' data in the repo.

**Don't build a `TuiScreen` variant speculatively.** Every existing variant was written against a real (redacted) transcript in this directory, not guessed remote prompt text. If a screen in here doesn't have a corresponding typed variant yet, that's expected — it renders via the `Unknown` fallback until someone asks for it to be modeled.
