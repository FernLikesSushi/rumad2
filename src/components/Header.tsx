import { Dog, LogOut } from "lucide-solid";
import { SettingsButton } from "./SettingsButton";
import { t } from "../i18n";
import { Show } from "solid-js";

export function Header(props: { disconnect?: () => Promise<void>; }) {
    const { disconnect, } = props;

    return <div class="grid grid-cols-3 items-start gap-2 w-full justify-center">
        <div class="justify-self-start" >
            <Show when={props.disconnect !== undefined}>
                <button type="button" class="btn btn-ghost btn-circle" aria-label={t().logout} onClick={disconnect}>
                    <LogOut />
                </button>
            </Show>
        </div>
        <h1 class="text-center justify-self-center text-xl">
            {t().title}
        </h1>
        <div class="justify-self-end">
            <SettingsButton />
        </div>
    </div>
} 