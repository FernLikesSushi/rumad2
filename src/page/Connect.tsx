import { createSignal } from "solid-js";
import { t } from "../i18n";
import { runAction } from "../api";
import { NoticeDialog } from "../components/NoticeDialog";
import { ConnectForm } from "../screens/ConnectForm";
import type { ClassifiedScreen, DialogBox } from "../types";

// The pre-connection page: owns the initial `connect` call and its own
// local busy/error state -- there's no TUI session yet for anything else
// to track. Once `connect` actually succeeds, this hands the resulting
// `ClassifiedScreen` up to `onConnected` and is done; `TuiRouter` takes
// over everything from there, and this component doesn't run again until
// the session ends and App.tsx remounts it.
export function Connect(props: { onConnected: (screen: ClassifiedScreen) => void }) {
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
      if (result) props.onConnected(result);
    } catch (err) {
      setDialogBox({ title: t().errorTitle, message: String(err) });
    } finally {
      setBusy(false);
    }
  }

  return (
    <>
      <NoticeDialog dialog={dialogBox()} onClose={() => setDialogBox(null)} />
      <ConnectForm onConnect={connect} busy={busy()} />
    </>
  );
}
