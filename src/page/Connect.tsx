import { createSignal, onCleanup, Show } from "solid-js";
import { useNavigate } from "@solidjs/router";
import { t } from "../i18n";
import { runAction } from "../screens/api";
import { NoticeDialog, createDialog } from "../components/NoticeDialog";
import { Header } from "../components/Header";
import { loadUsername, password } from "../data/username";
import { PawPrint } from "lucide-solid";
import { FadingText } from "../components/FadingText";
import { shuffledQuotes } from "../data/quotes";
import { createKeyboardListener } from "../components/KeyboardListener";

const LOKI_CODE = "loki";
const LOKI_TOAST_MS = 3000;

// The pre-connection page: owns the initial `connect` call and its own
// local busy/error state -- there's no TUI session yet for anything else
// to track. Once `connect` actually succeeds, this just navigates to
// "/session" and is done; it doesn't carry the resolved screen along --
// `TuiRouter` fetches its own starting state (`get_screen`) the moment it
// mounts rather than being handed it here.
export function Connect() {
  const navigate = useNavigate();
  const [busy, setBusy] = createSignal(false);
  const { dialog: dialog, show: showDialog, close: closeDialog } = createDialog();

  const [lokiMode, setLokiMode] = createSignal(false);
  const [showLokiToast, setShowLokiToast] = createSignal(false);

  let typedBuffer = "";
  let toastTimeout: ReturnType<typeof setTimeout>;

  // Easter egg: typing "loki" anywhere on this screen enables Loki mode.
  createKeyboardListener((key) => {
    if (key.length !== 1) return;
    typedBuffer = (typedBuffer + key.toLowerCase()).slice(-LOKI_CODE.length);
    if (typedBuffer !== LOKI_CODE) return;

    setLokiMode(lokiMode => !lokiMode);
    setShowLokiToast(lokiMode());
    clearTimeout(toastTimeout);
    toastTimeout = setTimeout(() => setShowLokiToast(false), LOKI_TOAST_MS);
  });

  onCleanup(() => clearTimeout(toastTimeout));

  async function connect(username: string, password: string) {
    setBusy(true);
    try {
      const result = await runAction({
        cmd: "connect",
        args: { username: username || undefined, password: password || undefined },
      });
      // "connect" always resolves to a real screen, never null (only
      // "disconnect" does -- see `runAction`).
      if (result) navigate("/session");
    } catch (err) {
      showDialog({ title: t().errorTitle, message: String(err) });
    } finally {
      setBusy(false);
    }
  }

  function submit(e: Event) {
    e.preventDefault();
    connect(loadUsername(), password());
  }



  return (
    <>
      <Header />
      <NoticeDialog dialog={dialog()} onClose={closeDialog} />

      <Show when={showLokiToast()}>
        <div class="toast toast-top toast-end z-50">
          <div class="alert alert-success">
            <span>Loki mode enabled.</span>
          </div>
        </div>
      </Show>

      <div class="flex flex-col items-center justify-center gap-4 flex-1">
        <Show when={lokiMode()}>
          <div class="badge badge-secondary">Loki Mode</div>
        </Show>
        <h2 class="font-semibold text-center">A friendly face to what you've already known.</h2>
        <h3 class="text-center">A fresh coat of paint on a rusty old machine.</h3>
        <h3 class="text-center">With a touch of modernity and a dash of nostalgia.</h3>
        <h2>
          <FadingText texts={shuffledQuotes} class="text-center" />
        </h2>
      </div>
      <div class="flex flex-col items-center justify-center gap-4 flex-1">
        <button type="submit" class="btn btn-outline btn-primary max-w-md" disabled={busy()} onClick={submit}>
          {busy() ? t().connecting : t().connect}
          <PawPrint />
        </button>
      </div>
    </>
  );
}
