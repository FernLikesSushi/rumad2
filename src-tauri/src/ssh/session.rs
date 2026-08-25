use anyhow::{bail, Context, Result};
use ssh2::Session;
use std::io::{ErrorKind, Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::time::{Duration, Instant};

use super::key::Key;

/// A logged-in SSH session driving an interactive PTY, with a virtual
/// terminal (`vt100`) mirroring what a real terminal would render. Unlike
/// the scripted, run-to-completion session this is adapted from, this one
/// stays open across Tauri commands: connect once, then drive it one
/// keystroke/line at a time as the user interacts with the app.
pub struct TuiSession {
    // Must outlive `channel`: ssh2 channels are only valid while their
    // parent session is alive, even though nothing here reads this field
    // directly after connecting.
    #[allow(dead_code)]
    session: Session,
    channel: ssh2::Channel,
    parser: vt100::Parser,
}

// ssh2's `Session`/`Channel` are not `Sync`, but they're only ever touched
// from behind the `Mutex` in Tauri's managed state, so `Send` is enough --
// Tauri commands may run on different pool threads but never concurrently
// for the same session.
unsafe impl Send for TuiSession {}

impl TuiSession {
    pub fn connect(
        addr: impl ToSocketAddrs,
        username: &str,
        password: &str,
        cols: u16,
        rows: u16,
    ) -> Result<Self> {
        let tcp = TcpStream::connect(addr).context("tcp connect failed")?;
        let mut session = Session::new().context("failed to create ssh session")?;
        session.set_tcp_stream(tcp);
        session.handshake().context("ssh handshake failed")?;
        session
            .userauth_password(username, password)
            .context("password auth failed")?;
        if !session.authenticated() {
            bail!("ssh authentication failed");
        }

        let mut channel = session
            .channel_session()
            .context("failed to open channel")?;
        channel
            .request_pty(
                "xterm-256color",
                None,
                Some((cols as u32, rows as u32, 0, 0)),
            )
            .context("failed to request pty")?;
        channel.shell().context("failed to start shell")?;

        // Bounds how long a single read blocks when no data is waiting;
        // acts as the polling interval for `settle`, not a hard cap.
        session.set_timeout(50);

        let mut tui = Self {
            session,
            channel,
            parser: vt100::Parser::new(rows, cols, 0),
        };
        // Let the login banner and initial menu finish rendering before
        // handing back the first screen.
        tui.settle(Duration::from_millis(800), Duration::from_secs(20))?;
        Ok(tui)
    }

    /// Plain-text rendering of the current virtual screen, ANSI codes
    /// stripped, one line per row.
    pub fn screen_text(&self) -> String {
        self.parser.screen().contents()
    }

    /// `Send` followed by `Key::Enter`, waiting for the resulting redraw
    /// to settle. Used for both raw text input and numbered menu choices.
    pub fn send_line(&mut self, text: &str) -> Result<()> {
        self.write_all(text.as_bytes())?;
        self.write_all(Key::Enter.as_bytes())?;
        self.settle(Duration::from_millis(500), Duration::from_secs(10))
    }

    /// Send raw text with no trailing Enter. Needed for fixed-field forms
    /// like the real student `Login` screen, which explicitly warns
    /// against pressing Enter while filling it in.
    pub fn send_text(&mut self, text: &str) -> Result<()> {
        self.write_all(text.as_bytes())?;
        self.settle(Duration::from_millis(500), Duration::from_secs(10))
    }

    /// Send a single non-printable key (arrows, Escape, Ctrl-C, ...).
    pub fn send_key(&mut self, key: Key) -> Result<()> {
        self.write_all(key.as_bytes())?;
        self.settle(Duration::from_millis(500), Duration::from_secs(10))
    }

    /// Drain any output that's arrived since the last read, without
    /// sending anything -- lets a caller poll for a redraw that's still
    /// settling asynchronously (e.g. `TuiScreen::Processing`).
    pub fn refresh(&mut self) -> Result<()> {
        self.settle(Duration::from_millis(500), Duration::from_secs(10))
    }

    /// Feed in whatever's already waiting on the channel, without blocking
    /// to wait for more. Used by the long-lived background watcher
    /// (`commands::exec::spawn_screen_watcher`) that ticks continuously
    /// rather than waiting on one specific redraw the way `settle` does.
    pub(crate) fn drain_available(&mut self) -> Result<bool> {
        let mut changed = false;
        while self.pump_once()? {
            changed = true;
        }
        Ok(changed)
    }

    /// Best-effort logout: the menu system is exited via its own "0"
    /// option rather than closing the socket out from under it. Errors are
    /// swallowed since the caller is tearing the session down regardless.
    pub fn close(mut self) {
        let _ = self.send_line("0");
    }

    /// Whether the remote has actually closed its end of the channel --
    /// the only reliable signal that the session is truly over. Screen
    /// *text* alone (e.g. the "PROCESO CONCLUIDO" banner) isn't enough:
    /// it can apparently show up without the connection actually closing.
    /// Reflects `channel.eof()`, which libssh2 sets as a side effect of a
    /// read actually reaching end-of-stream -- so this is only meaningful
    /// right after a pump (`pump_once`/`settle`/`drain_available`) has
    /// run, not as an independent poll.
    pub(crate) fn is_closed(&self) -> bool {
        self.channel.eof()
    }

    fn write_all(&mut self, bytes: &[u8]) -> Result<()> {
        self.channel
            .write_all(bytes)
            .context("failed to write to channel")?;
        self.channel.flush().context("failed to flush channel")?;
        Ok(())
    }

    /// Read one chunk if available, feeding it into the virtual terminal.
    /// Returns `false` once no more data shows up within the poll window.
    fn pump_once(&mut self) -> Result<bool> {
        let mut buf = [0u8; 4096];
        match self.channel.read(&mut buf) {
            Ok(0) => Ok(false),
            Ok(n) => {
                self.parser.process(&buf[..n]);
                Ok(true)
            }
            Err(e) if e.kind() == ErrorKind::TimedOut || e.kind() == ErrorKind::WouldBlock => {
                Ok(false)
            }
            Err(e) => Err(e).context("failed to read from channel"),
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
    fn settle(&mut self, quiet_for: Duration, overall: Duration) -> Result<()> {
        let deadline = Instant::now() + overall;
        let mut quiet_since: Option<Instant> = None;
        loop {
            if self.pump_once()? || self.parser.screen().hide_cursor() {
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
        }
    }
}
