import { LogOut, Settings } from "lucide-solid";
import { t } from "../i18n";

export function SettingsButton({ }) {
    // TODO: localize the aria-label for the button, and add a click handler to open the settings modal
    // TODO: Settings modal
    return <button type="button" class="btn btn-ghost btn-circle" aria-label={"Settings"} onClick={() => {}}>
        <Settings />
    </button>;
}
