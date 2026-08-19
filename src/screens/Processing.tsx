import { t } from "../i18n";

// No actions here, unlike every other screen -- the backend's own
// `spawn_screen_watcher` (`commands/exec.rs`) keeps polling in the
// background and pushes a `screen-changed` event once the remote finishes
// computing and redraws on its own; `App.tsx`'s listener swaps this out
// for the real result automatically, no user action needed.
export function ProcessingScreen() {
  return (
    <>
      <h2>{t().processingTitle}</h2>
    </>
  );
}
