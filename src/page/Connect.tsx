import { createSignal } from "solid-js";
import { useNavigate } from "@solidjs/router";
import { t } from "../i18n";
import { runAction } from "../api";
import { NoticeDialog } from "../components/NoticeDialog";
import type { DialogBox } from "../types";
import { Header } from "../components/Header";
import { loadUsername, password } from "../username";
import { PawPrint } from "lucide-solid";

// The pre-connection page: owns the initial `connect` call and its own
// local busy/error state -- there's no TUI session yet for anything else
// to track. Once `connect` actually succeeds, this just navigates to
// "/session" and is done; it doesn't carry the resolved screen along --
// `TuiRouter` fetches its own starting state (`get_screen`) the moment it
// mounts rather than being handed it here.
export function Connect() {
  const navigate = useNavigate();
  const [busy, setBusy] = createSignal(false);
  const [dialogBox, setDialogBox] = createSignal<DialogBox | null>(null);

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
      setDialogBox({ title: t().errorTitle, message: String(err) });
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
      <NoticeDialog dialog={dialogBox()} onClose={() => setDialogBox(null)} />
      <div class="flex flex-col items-center justify-center gap-4 flex-1">
        <button type="submit" class="btn btn-outline btn-primary max-w-md" disabled={busy()} onClick={submit}>
          {busy() ? t().connecting : t().connect}
          <PawPrint />
        </button>
      </div>
    </>
  );
}
