//! Integration tests that hit the real, live RUMAD system
//! (`config::DEFAULT_HOST`) over an actual SSH connection -- a canary for
//! the remote changing its screen format under us, not a general test
//! harness. Opt-in only: every test here is `#[ignore]`d so a plain
//! `cargo test` (per CLAUDE.md) never touches the network or a third
//! party's server. Run them explicitly with:
//!
//!   cargo test --test production -- --ignored
//!
//! `#[tokio::test]`, not `#[test]` -- `TuiSession`/`RumadScreen` are
//! `async fn` (russh, this app's SSH library, is async-only), so this
//! integration crate needs its own runtime to drive them, same as
//! `commands::exec::act` does inside the app itself.
//!
//! Drives screens through `RumadScreen` (`select`/`line`/`exit`/
//! `continue_screen`), the same trait `commands::interact::send` uses --
//! that trait (and `TuiScreen::as_rumad_screen`) is `pub`, not
//! `pub(crate)`, specifically so this external integration crate can see
//! it, rather than this file reimplementing screen interaction by poking
//! `TuiSession`'s raw primitives itself.
//!
//! Most tests here use only `config::DEFAULT_USERNAME`/`DEFAULT_PASSWORD`
//! -- the shared, unauthenticated "estudiante" account, empty password,
//! público by design (see `config`'s own doc comment) -- and only
//! read-only navigation (menu selection, course-schedule search, itself
//! public information). None of them ever attempt Altas/Bajas/Cambio or
//! anything else that could mutate a real student's enrollment.
//!
//! `reaches_login_screen_with_real_account` is the one exception: it
//! needs a real student's own SSH-level credentials (the shared account
//! gets turned away before ever reaching `Login` -- see that test's own
//! doc comment), supplied only via env vars the *runner* sets locally,
//! never hardcoded here. It stops at classifying the `Login` screen and
//! backs out immediately -- it never actually submits `Login`'s 4 fields,
//! so no real login attempt (successful or not) ever happens.

use rumad_2_lib::config;
use rumad_2_lib::screens::{self, Dialog, MenuKind, RumadScreen, SearchKind, TuiScreen};
use rumad_2_lib::ssh::session::TuiSession;

async fn connect_as_shared_account() -> TuiSession {
    TuiSession::connect(
        (config::DEFAULT_HOST, config::SSH_PORT),
        config::DEFAULT_USERNAME,
        config::DEFAULT_PASSWORD,
        80,
        24,
    )
    .await
    .expect("failed to connect to the live RUMAD system")
}

/// Every screen but `Notice`/`Disconnected` implements `RumadScreen` (see
/// `TuiScreen::as_rumad_screen`'s own doc comment) -- none of the screens
/// these tests navigate through are either.
fn interactive(screen: &TuiScreen) -> &dyn RumadScreen {
    screen
        .as_rumad_screen()
        .expect("screen should support RumadScreen interaction")
}

/// Re-classifies the current screen, polling via `TuiSession::refresh`
/// (the same primitive `commands::exec::spawn_screen_watcher` ticks on in
/// the real app) while the remote is still showing `Dialog::Processing`
/// ("Programa en Proceso") rather than trusting the first read -- the
/// preceding `send_line`/`send_text`'s own `settle()` can return while
/// that marquee is still up (confirmed live: exactly the race
/// `ssh/session.rs`'s cursor-visibility fix targets, just not eliminated
/// entirely). 20 attempts * up to `refresh()`'s own ~10s cap is a
/// generous ceiling; every real observed case here settles in one or two.
async fn wait_until_settled(session: &mut TuiSession) -> screens::ClassifiedScreen {
    let mut classified = screens::classify(&session.screen_text());
    let mut attempts = 0;
    while matches!(classified.dialog, Some(Dialog::Processing)) && attempts < 20 {
        session
            .refresh()
            .await
            .expect("failed to poll for a settled screen");
        classified = screens::classify(&session.screen_text());
        attempts += 1;
    }
    classified
}

#[tokio::test]
#[ignore = "hits the real production RUMAD system over the network -- opt in with `cargo test --test production -- --ignored`"]
async fn connects_and_classifies_main_menu() {
    let session = connect_as_shared_account().await;
    let classified = screens::classify(&session.screen_text());

    match &classified.screen {
        TuiScreen::Menu(menu) if menu.menu == MenuKind::MainMenu => {}
        other => panic!("expected the shared account's MainMenu, got {other:?}"),
    }

    session.close().await;
}

#[tokio::test]
#[ignore = "hits the real production RUMAD system over the network -- opt in with `cargo test --test production -- --ignored`"]
async fn main_menu_matricula_option_rejected_for_shared_account() {
    let mut session = connect_as_shared_account().await;
    let classified = screens::classify(&session.screen_text());

    // "2. Seleccion de Secciones (Matricula)" -- account-specific, so the
    // shared account gets turned away rather than ever reaching the real
    // `Login` screen. See `screens/mod.rs`'s top doc comment.
    interactive(&classified.screen)
        .select(&mut session, "2")
        .await
        .expect("failed to select the Matricula option");
    let classified = screens::classify(&session.screen_text());

    let message = match &classified.dialog {
        Some(Dialog::Notice { message, .. }) => message.clone(),
        other => panic!("expected a rejection Notice, got dialog: {other:?}"),
    };
    assert!(
        message.to_lowercase().contains("no esta disponible")
            || message.to_lowercase().contains("no está disponible"),
        "expected the shared-account rejection notice, got: {message}"
    );
    assert!(
        classified.or_err().is_err(),
        "this specific rejection should be promoted to an Err by or_err()"
    );

    session.close().await;
}

#[tokio::test]
#[ignore = "hits the real production RUMAD system over the network -- opt in with `cargo test --test production -- --ignored`"]
async fn searches_course_schedule_and_continues() {
    let mut session = connect_as_shared_account().await;
    let classified = screens::classify(&session.screen_text());

    // MainMenu -- "5. Ver otra informacion" -> MenuDespliegue.
    interactive(&classified.screen)
        .select(&mut session, "5")
        .await
        .expect("failed to open MenuDespliegue");
    let classified = screens::classify(&session.screen_text());
    match &classified.screen {
        TuiScreen::Menu(menu) if menu.menu == MenuKind::MenuDespliegue => {}
        other => panic!("expected MenuDespliegue, got {other:?}"),
    }

    // MenuDespliegue -- "6. Horario de cursos disponibles en Matricula" ->
    // HorarioSemester.
    interactive(&classified.screen)
        .select(&mut session, "6")
        .await
        .expect("failed to open Horario de cursos disponibles");
    let classified = screens::classify(&session.screen_text());
    match &classified.screen {
        TuiScreen::Menu(menu) if menu.menu == MenuKind::HorarioSemester => {}
        other => panic!("expected HorarioSemester, got {other:?}"),
    }

    // Pick a semester -- "1=1erVer" -> Search(HorarioCurso).
    interactive(&classified.screen)
        .select(&mut session, "1")
        .await
        .expect("failed to pick a semester");
    let classified = screens::classify(&session.screen_text());
    match &classified.screen {
        TuiScreen::Search(search) if search.search == SearchKind::HorarioCurso => {}
        other => panic!("expected Search(HorarioCurso), got {other:?}"),
    }

    // A bare subject prefix (not a full course code) reliably matches
    // multiple real sections without depending on any one course/section
    // still existing a given semester -- read-only, publicly published
    // schedule data, not tied to any student's own enrollment.
    interactive(&classified.screen)
        .line(&mut session, "HIST")
        .await
        .expect("failed to submit the course search");
    let classified = wait_until_settled(&mut session).await;
    assert!(
        matches!(&classified.screen, TuiScreen::CourseResults(_)),
        "expected CourseResults, got {:?}",
        classified.screen
    );
    assert!(classified.can_continue, "CourseResults should report can_continue");

    // "Enter to continue" -- confirms the keystroke is accepted rather
    // than left dangling.
    interactive(&classified.screen)
        .continue_screen(&mut session)
        .await
        .expect("failed to send the continue keystroke");
    let classified = wait_until_settled(&mut session).await;
    assert!(
        matches!(&classified.screen, TuiScreen::CourseResults(_) | TuiScreen::Menu(_)),
        "expected to still be on a recognized screen after continuing, got {:?}",
        classified.screen
    );

    session.close().await;
}

#[tokio::test]
#[ignore = "hits the real production RUMAD system over the network -- opt in with `cargo test --test production -- --ignored`"]
async fn course_search_rejects_nonexistent_course() {
    let mut session = connect_as_shared_account().await;
    let classified = screens::classify(&session.screen_text());
    interactive(&classified.screen)
        .select(&mut session, "5")
        .await
        .expect("failed to open MenuDespliegue");
    let classified = screens::classify(&session.screen_text());
    interactive(&classified.screen)
        .select(&mut session, "6")
        .await
        .expect("failed to open Horario de cursos disponibles");
    let classified = screens::classify(&session.screen_text());
    interactive(&classified.screen)
        .select(&mut session, "1")
        .await
        .expect("failed to pick a semester");
    let classified = screens::classify(&session.screen_text());

    // Not a real subject prefix -- exercises the "invalid subject"
    // rejection without querying (let alone touching) anything real. A
    // real-but-made-up catalog number (e.g. "HIST9999") turned out *not*
    // to trigger this: confirmed live, the search only ever looks at the
    // subject part ("Puede indicar solo MATERIA"), landing on
    // `HorarioSeccion` to narrow down among every real HIST section
    // instead of rejecting anything.
    interactive(&classified.screen)
        .line(&mut session, "ZZZZ")
        .await
        .expect("failed to submit the bogus course search");
    let classified = wait_until_settled(&mut session).await;

    let message = match &classified.dialog {
        Some(Dialog::Notice { message, .. }) => message.clone(),
        other => panic!("expected a rejection Notice, got dialog: {other:?}"),
    };
    assert!(
        message.to_uppercase().contains("MATERIA INCORRECTA"),
        "expected an \"invalid subject\" notice, got: {message}"
    );

    // This rejection is *not* one `or_err` promotes to an `Err` -- the
    // search prompt is still live underneath it (see `screens/mod.rs`'s
    // classification-priority doc comment), so exit out via the trait
    // rather than leaving the session sitting on it.
    interactive(&classified.screen)
        .exit(&mut session)
        .await
        .expect("failed to exit the search prompt");

    session.close().await;
}

#[tokio::test]
#[ignore = "hits the real production RUMAD system over the network -- opt in with `cargo test --test production -- --ignored`"]
async fn exits_menu_despliegue_back_to_main_menu() {
    let mut session = connect_as_shared_account().await;
    let classified = screens::classify(&session.screen_text());
    interactive(&classified.screen)
        .select(&mut session, "5")
        .await
        .expect("failed to open MenuDespliegue");
    let classified = screens::classify(&session.screen_text());

    // "0. Finalizar" -- MenuDespliegue's own exit key, confirmed live (see
    // `RumadScreen::exit`'s doc comment).
    interactive(&classified.screen)
        .exit(&mut session)
        .await
        .expect("failed to exit MenuDespliegue");
    let classified = screens::classify(&session.screen_text());
    match &classified.screen {
        TuiScreen::Menu(menu) if menu.menu == MenuKind::MainMenu => {}
        other => panic!("expected to be back at MainMenu, got {other:?}"),
    }

    session.close().await;
}

#[tokio::test]
#[ignore = "hits the real production RUMAD system over the network, AND needs a real student's own SSH credentials -- opt in with `cargo test --test production -- --ignored`, with RUMAD_TEST_SSH_USER/RUMAD_TEST_SSH_PASSWORD set"]
async fn reaches_login_screen_with_real_account() {
    let (username, password) = match (
        std::env::var("RUMAD_TEST_SSH_USER"),
        std::env::var("RUMAD_TEST_SSH_PASSWORD"),
    ) {
        (Ok(u), Ok(p)) => (u, p),
        _ => {
            // No credentials to work with -- pass trivially rather than
            // fail, this test only makes sense when someone deliberately
            // opts into supplying their own real account.
            eprintln!("skipping: set RUMAD_TEST_SSH_USER/RUMAD_TEST_SSH_PASSWORD to run this test");
            return;
        }
    };

    let mut session = TuiSession::connect((config::DEFAULT_HOST, config::SSH_PORT), &username, &password, 80, 24)
        .await
        .expect("failed to connect with the provided real account");
    let classified = screens::classify(&session.screen_text());

    interactive(&classified.screen)
        .select(&mut session, "2")
        .await
        .expect("failed to select the Matricula option");
    let classified = screens::classify(&session.screen_text());
    let login = match &classified.screen {
        TuiScreen::Login(login) => login,
        other => panic!("expected the Login screen, got {other:?}"),
    };
    assert_eq!(login.fields.len(), 4, "expected Login's 4 fixed fields");

    // Deliberately never calls `LoginScreen::login` -- this only confirms
    // the page is reachable and classifies correctly, not a real login
    // attempt. Back out the same way every other screen does.
    login.exit(&mut session).await.expect("failed to exit Login");

    session.close().await;
}
