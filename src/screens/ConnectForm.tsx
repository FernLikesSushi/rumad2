import { createEffect, createSignal } from "solid-js";
import { t } from "../i18n";
import { loadUsername, saveUsername } from "../username";

export function ConnectForm(props: { onConnect: (username: string, password: string) => void; busy: boolean }) {
  const [username, setUsername] = createSignal(loadUsername());
  const [password, setPassword] = createSignal("");

  createEffect(() => {
    saveUsername(username());
  });

  function submit(e: Event) {
    e.preventDefault();
    props.onConnect(username(), password());
  }

  return (
    <form class="flex flex-col gap-2.5 max-w-lg mx-auto text-left" onSubmit={submit}>
      <p class="text-[0.85em] opacity-75">{t().loginHint}</p>
      <input
        class="input"
        placeholder={t().usernamePlaceholder}
        value={username()}
        onInput={(e) => setUsername(e.currentTarget.value)}
      />
      <input
        type="password"
        class="input"
        placeholder={t().passwordPlaceholder}
        value={password()}
        onInput={(e) => setPassword(e.currentTarget.value)}
      />
      <button type="submit" class="btn btn-outline btn-primary" disabled={props.busy}>
        {props.busy ? t().connecting : t().connect}
      </button>
    </form>
  );
}
