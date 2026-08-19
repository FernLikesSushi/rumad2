import { t } from "../i18n";

export function ConnectForm(props: {
  username: string;
  password: string;
  onUsernameInput: (value: string) => void;
  onPasswordInput: (value: string) => void;
  onSubmit: (e: Event) => void;
  busy: boolean;
}) {
  return (
    <form class="login" onSubmit={props.onSubmit}>
      <p class="hint">{t().loginHint}</p>
      <input
        placeholder={t().usernamePlaceholder}
        value={props.username}
        onInput={(e) => props.onUsernameInput(e.currentTarget.value)}
      />
      <input
        type="password"
        placeholder={t().passwordPlaceholder}
        value={props.password}
        onInput={(e) => props.onPasswordInput(e.currentTarget.value)}
      />
      <button type="submit" disabled={props.busy}>
        {props.busy ? t().connecting : t().connect}
      </button>
    </form>
  );
}
