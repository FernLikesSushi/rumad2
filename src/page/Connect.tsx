import { createEffect, createSignal, onCleanup, Show } from "solid-js";
import { useNavigate } from "@solidjs/router";
import { t } from "../i18n";
import { Header } from "../components/Header";
import { loadUsername, password } from "../data/username";
import { PawPrint } from "lucide-solid";
import { FadingText } from "../components/FadingText";
import { shuffledQuotes } from "../data/quotes";
import { createKeyboardListener } from "../components/KeyboardListener";
import { busy, connected, connect as connectSession } from "../data/tui";
import { Footer } from "../components/Footer";

const LOKI_CODE = "loki";
const LOKI_TOAST_MS = 3000;

// The pre-connection page: kicks off `connect` through `data/tui.ts`'s
// shared resource, same as every other session action -- errors surface
// via the global `TuiDialogHandler` on their own, and a successful connect
// flips `connected()` to true, so this just reacts to that to navigate
// instead of awaiting the call itself.
export function Connect() {
  const navigate = useNavigate();

  const [lokiMode, setLokiMode] = createSignal(false);
  const [showLokiToast, setShowLokiToast] = createSignal(false);

  let typedBuffer = "";
  let toastTimeout: ReturnType<typeof setTimeout>;

  // Easter egg: typing "loki" anywhere on this screen enables Loki mode.
  createKeyboardListener((key) => {
    if (key.length !== 1) return;
    typedBuffer = (typedBuffer + key.toLowerCase()).slice(-LOKI_CODE.length);
    if (typedBuffer !== LOKI_CODE) return;

    setLokiMode((lokiMode) => !lokiMode);
    setShowLokiToast(lokiMode());
    clearTimeout(toastTimeout);
    toastTimeout = setTimeout(() => setShowLokiToast(false), LOKI_TOAST_MS);
  });

  onCleanup(() => clearTimeout(toastTimeout));

  createEffect(() => {
    if (connected()) navigate("/session");
  });

  function submit(e: Event) {
    e.preventDefault();
    connectSession(loadUsername(), password());
  }

  return (
    <>
      <Header />

      <Show when={showLokiToast()}>
        <div class="toast toast-top toast-end z-50">
          <div class="alert alert-success">
            <span>{t().lokiModeToast}</span>
          </div>
        </div>
      </Show>

      <div class="flex flex-col items-center justify-center gap-2 sm:gap-4 flex-1 min-h-0">
        <Show when={lokiMode()}>
          <div class="badge badge-secondary">{t().lokiModeBadge}</div>
        </Show>
        <h2 class="font-semibold text-center max-sm:my-1">{t().connectTagline1}</h2>
        <h3 class="text-center max-sm:my-1">{t().connectTagline2}</h3>
        <h3 class="text-center max-sm:my-1">{t().connectTagline3}</h3>
        <h2 class="max-sm:my-1">
          <FadingText texts={shuffledQuotes} class="text-center" />
        </h2>
      </div>
      <div class="flex flex-col items-center justify-center gap-2 sm:gap-4 flex-1 min-h-0">
        <button
          type="submit"
          class="btn btn-outline btn-primary max-w-md"
          disabled={busy()}
          onClick={submit}
        >
          {busy() ? t().connecting : t().connect}
          <PawPrint />
        </button>
      </div>
      <Footer />
    </>
  );
}
