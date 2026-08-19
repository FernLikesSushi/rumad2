import { t } from "../i18n";

export function DisconnectedScreen(props: { busy: boolean; onReconnect: () => void }) {
  return (
    <>
      <h2>{t().disconnectedTitle}</h2>
      <p class="hint">{t().disconnectedHint}</p>
      <button disabled={props.busy} onClick={props.onReconnect}>
        {t().reconnect}
      </button>
    </>
  );
}
