import { t } from "../i18n";
import { loadUsername, password } from "../username";
import { PawPrint } from "lucide-solid";

export function ConnectForm(props: { onConnect: (username: string, password: string) => void; busy: boolean }) {

  function submit(e: Event) {
    e.preventDefault();
    props.onConnect(loadUsername(), password());
  }

  return (
    <button type="submit" class="btn btn-outline btn-primary" disabled={props.busy} onClick={submit}>
      {props.busy ? t().connecting : t().connect}
      <PawPrint />
    </button>
  );
}
