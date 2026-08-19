import { createSignal } from "solid-js";
import { t } from "../i18n";

export function ConnectForm(props: { onConnect: (username: string, password: string) => void; busy: boolean }) {
  const [username, setUsername] = createSignal("");
  const [password, setPassword] = createSignal("");

  function submit(e: Event) {
    e.preventDefault();
    props.onConnect(username(), password());
  }

  return (
    <form class="login" onSubmit={submit}>
      <p class="hint">{t().loginHint}</p>
      <input
        placeholder={t().usernamePlaceholder}
        value={username()}
        onInput={(e) => setUsername(e.currentTarget.value)}
      />
      <input
        type="password"
        placeholder={t().passwordPlaceholder}
        value={password()}
        onInput={(e) => setPassword(e.currentTarget.value)}
      />
      <button type="submit" disabled={props.busy}>
        {props.busy ? t().connecting : t().connect}
      </button>
    </form>
  );
}
