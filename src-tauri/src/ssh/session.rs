use anyhow::{bail, Context, Result};
use russh::client::{self, Msg};
use russh::keys::PublicKeyOrCertificate;
use russh::{mac, Channel, ChannelMsg, Disconnect, Preferred};
use std::borrow::Cow;
use std::net::ToSocketAddrs;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::time::sleep;

use super::key::Key;

/// The real remote (an old OpenVMS sshd) only offers legacy MAC
/// algorithms -- confirmed live: key exchange fails outright with
/// `russh`'s plain defaults ("No common Mac algorithm - ours: [...
/// hmac-sha2-... only], theirs: [hmac-sha1, hmac-sha256@ssh.com]"), since
/// `russh` excludes SHA-1 variants from its default preference list on
/// security grounds. `hmac-sha1` is weak by modern standards, but this
/// app already trusts this exact host unconditionally (`ServerAcceptsAny`
/// below) -- accepting its one working legacy MAC to actually reach it is
/// consistent with that, not a separate new tradeoff.
fn preferred_algorithms() -> Preferred {
    Preferred {
        mac: Cow::Owned(
            Preferred::DEFAULT
                .mac
                .iter()
                .copied()
                .chain([mac::HMAC_SHA1])
                .collect(),
        ),
        ..Preferred::DEFAULT
    }
}

struct ServerAcceptsAny;

impl client::Handler for ServerAcceptsAny {
    type Error = russh::Error;

    /// Accepts any host key -- `russh` refuses every key by default unless
    /// a `Handler` explicitly opts in. Not verifying means no protection
    /// against a man-in-the-middle substituting a different host key, but
    /// this app already trusts this exact host unconditionally (see
    /// `preferred_algorithms`'s doc comment), so this isn't a separate new
    /// tradeoff.
    async fn check_server_key(
        &mut self,
        _server_public_key: &PublicKeyOrCertificate,
    ) -> std::result::Result<bool, Self::Error> {
        Ok(true)
    }
}

/// A logged-in SSH session driving an interactive PTY, with a virtual
/// terminal (`vt100`) mirroring what a real terminal would render. Unlike
/// the scripted, run-to-completion session this is adapted from, this one
/// stays open across Tauri commands: connect once, then drive it one
/// keystroke/line at a time as the user interacts with the app.
pub struct TuiSession {
    handle: client::Handle<ServerAcceptsAny>,
    channel: Channel<Msg>,
    parser: vt100::Parser,
    // `channel.wait()` has no separate "did the remote close" accessor, so
    // this caches whatever `ChannelMsg::Eof`/`Close` a pump last observed
    // instead -- see `is_closed`'s own doc comment for the "only
    // meaningful right after a pump has run" contract this implies.
    closed: bool,
}

impl TuiSession {
    pub async fn connect(
        addr: impl ToSocketAddrs,
        username: &str,
        password: &str,
        cols: u16,
        rows: u16,
    ) -> Result<Self> {
        let addr = addr
            .to_socket_addrs()
            .context("failed to resolve host")?
            .next()
            .context("host resolved to no addresses")?;

        let config = client::Config {
            preferred: preferred_algorithms(),
            ..Default::default()
        };
        let mut handle = client::connect(Arc::new(config), addr, ServerAcceptsAny)
            .await
            .context("ssh connect failed")?;

        let auth = handle
            .authenticate_password(username, password)
            .await
            .context("password auth failed")?;
        if !auth.success() {
            bail!("ssh authentication failed");
        }

        let channel = handle
            .channel_open_session()
            .await
            .context("failed to open channel")?;
        channel
            .request_pty(true, "xterm-256color", cols as u32, rows as u32, 0, 0, &[])
            .await
            .context("failed to request pty")?;
        channel
            .request_shell(true)
            .await
            .context("failed to start shell")?;

        let mut tui = Self {
            handle,
            channel,
            parser: vt100::Parser::new(rows, cols, 0),
            closed: false,
        };
        // Let the login banner and initial menu finish rendering before
        // handing back the first screen.
        tui.settle(Duration::from_millis(800), Duration::from_secs(20))
            .await?;
        Ok(tui)
    }

    /// Plain-text rendering of the current virtual screen, ANSI codes
    /// stripped, one line per row.
    pub fn screen_text(&self) -> String {
        self.parser.screen().contents()
    }

    /// `Send` followed by `Key::Enter`, waiting for the resulting redraw
    /// to settle. Used for both raw text input and numbered menu choices.
    pub async fn send_line(&mut self, text: &str) -> Result<()> {
        self.write_all(text.as_bytes()).await?;
        self.write_all(Key::Enter.as_bytes()).await?;
        self.settle(Duration::from_millis(500), Duration::from_secs(10))
            .await
    }

    /// Send raw text with no trailing Enter. Needed for fixed-field forms
    /// like the real student `Login` screen, which explicitly warns
    /// against pressing Enter while filling it in.
    pub async fn send_text(&mut self, text: &str) -> Result<()> {
        self.write_all(text.as_bytes()).await?;
        self.settle(Duration::from_millis(500), Duration::from_secs(10))
            .await
    }

    /// Send a single non-printable key (arrows, Escape, Ctrl-C, ...).
    pub async fn send_key(&mut self, key: Key) -> Result<()> {
        self.write_all(key.as_bytes()).await?;
        self.settle(Duration::from_millis(500), Duration::from_secs(10))
            .await
    }

    /// Drain any output that's arrived since the last read, without
    /// sending anything -- lets a caller poll for a redraw that's still
    /// settling asynchronously (e.g. `TuiScreen::Processing`).
    pub async fn refresh(&mut self) -> Result<()> {
        self.settle(Duration::from_millis(500), Duration::from_secs(10))
            .await
    }

    /// Feed in whatever's already waiting on the channel, without blocking
    /// to wait for more. Used by the long-lived background watcher
    /// (`commands::exec::spawn_screen_watcher`) that ticks continuously
    /// rather than waiting on one specific redraw the way `settle` does.
    pub(crate) async fn drain_available(&mut self) -> Result<bool> {
        let mut changed = false;
        while self.pump_once().await? {
            changed = true;
        }
        Ok(changed)
    }

    /// Best-effort logout: the menu system is exited via its own "0"
    /// option rather than closing the socket out from under it. Errors are
    /// swallowed since the caller is tearing the session down regardless.
    pub async fn close(mut self) {
        let _ = self.send_line("0").await;
        let _ = self
            .handle
            .disconnect(Disconnect::ByApplication, "", "English")
            .await;
    }

    /// Whether the remote has actually closed its end of the channel --
    /// the only reliable signal that the session is truly over. Screen
    /// *text* alone (e.g. the "PROCESO CONCLUIDO" banner) isn't enough:
    /// it can apparently show up without the connection actually closing.
    /// Reflects the last `ChannelMsg::Eof`/`Close` a pump observed -- so
    /// this is only meaningful right after a pump (`pump_once`/`settle`/
    /// `drain_available`) has run, not as an independent poll.
    pub(crate) fn is_closed(&self) -> bool {
        self.closed
    }

    async fn write_all(&mut self, bytes: &[u8]) -> Result<()> {
        self.channel
            .data_bytes(bytes.to_vec())
            .await
            .context("failed to write to channel")?;
        Ok(())
    }

    /// Read one chunk if available, feeding it into the virtual terminal.
    /// Returns `false` once no more data shows up within the poll window
    /// (50ms).
    async fn pump_once(&mut self) -> Result<bool> {
        let msg = tokio::time::timeout(Duration::from_millis(50), self.channel.wait()).await;
        match msg {
            Ok(Some(ChannelMsg::Data { data })) => {
                self.parser.process(&data);
                Ok(true)
            }
            // Not fed to the parser -- this app's own `send_text`/
            // `send_line` never write to a separate stderr-like stream.
            Ok(Some(ChannelMsg::ExtendedData { .. })) => Ok(true),
            Ok(Some(ChannelMsg::Eof | ChannelMsg::Close)) => {
                self.closed = true;
                Ok(true)
            }
            Ok(Some(_)) => Ok(true),
            Ok(None) => {
                self.closed = true;
                Ok(false)
            }
            Err(_elapsed) => Ok(false),
        }
    }

    /// Drain incoming output until the remote goes quiet for `quiet_for`
    /// with the cursor visible, or `overall` elapses. Stands in for "wait
    /// until the next screen has finished rendering" without needing to
    /// know what that screen is -- the app doesn't follow a fixed script,
    /// so it can't wait for a specific expected pattern.
    ///
    /// A byte-silence gap alone isn't reliable on its own: a redraw can
    /// legitimately pause mid-draw (server-side latency) for longer than
    /// `quiet_for`, which would otherwise make this return before the
    /// screen is actually finished. Full-screen VT100 apps -- OpenVMS
    /// SMG$-based ones included -- near-universally hide the cursor
    /// (DECTCEM, `ESC[?25l`) while repainting and show it again
    /// (`ESC[?25h`) only once done and parked at the next input field, so
    /// `hide_cursor()` doubles as a real "still mid-draw" signal: a quiet
    /// gap while it's still hidden doesn't count toward `quiet_for`. If the
    /// remote never toggles it at all, `hide_cursor()` just stays `false`
    /// throughout and this behaves exactly like the old silence-only check
    /// -- `overall` is still the hard cap either way.
    async fn settle(&mut self, quiet_for: Duration, overall: Duration) -> Result<()> {
        let deadline = Instant::now() + overall;
        let mut quiet_since: Option<Instant> = None;
        loop {
            if self.pump_once().await? || self.parser.screen().hide_cursor() {
                quiet_since = None;
            } else {
                let start = *quiet_since.get_or_insert_with(Instant::now);
                if Instant::now() - start > quiet_for {
                    return Ok(());
                }
            }
            if Instant::now() >= deadline {
                return Ok(());
            }
            // `pump_once`'s own 50ms timeout already paces this loop when
            // data's arriving; this only matters once the channel is fully
            // quiet (`Ok(None)`/closed), so it doesn't spin hot waiting for
            // `overall` to elapse.
            if self.closed {
                sleep(Duration::from_millis(50)).await;
            }
        }
    }
}
