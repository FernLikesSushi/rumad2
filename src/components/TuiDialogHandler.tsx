import { createEffect } from "solid-js";
import { t } from "../i18n";
import { response } from "../data/tui";
import { NoticeDialog, createDialog } from "./NoticeDialog";

// Watches `data/tui.ts`'s `response` and pops the error/notice dialog --
// split out from `tui.ts` itself (a plain data module) since showing a
// dialog is a UI concern, not session state. Rendered once, globally (see
// App.tsx), not just while "/session" is mounted -- an action dispatched
// from anywhere can still fail while the user is on a different route,
// now that `data/tui.ts`'s state persists across navigation.
export function TuiDialogHandler() {
  const { dialog, show, close } = createDialog();

  createEffect(() => {
    if (response.loading) return;
    const err = response.error;
    if (err) {
      show({ title: t().errorTitle, message: String(err) });
      return;
    }
    const result = response();
    if (result?.dialog?.kind === "Notice") {
      show({ title: t().noticeTitle, message: result.dialog.message });
    }
  });

  return <NoticeDialog dialog={dialog()} onClose={close} />;
}
