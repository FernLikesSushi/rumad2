import { t } from "../i18n";

export function DisconnectedScreen(props: { busy: boolean; onReconnect: () => void }) {
  return (
    <>
      <h2>{t().disconnectedTitle}</h2>
      <p class="text-[0.85em] opacity-75">{t().disconnectedHint}</p>
      <button class="btn btn-outline btn-primary" disabled={props.busy} onClick={props.onReconnect}>
        {t().reconnect}
      </button>
    </>
  );
}
