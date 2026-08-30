import { Dog, LogOut, Menu } from "lucide-solid";
import { SettingsButton } from "./SettingsButton";
import { DRAWER_ID } from "./Drawer";
import { t } from "../i18n";
import { Show } from "solid-js";

export function Header(props: { disconnect?: () => Promise<void>; }) {
    const { disconnect, } = props;

    return <div class="grid grid-cols-3 items-start gap-2 w-full justify-center">
        <div class="justify-self-start flex items-center gap-1">
            <label
                for={DRAWER_ID}
                aria-label={t().openMenu}
                class="btn btn-ghost btn-circle hidden sm:inline-flex lg:hidden tooltip tooltip-bottom tooltip-start"
                data-tip={t().openMenu}
            >
                <Menu />
            </label>

            <Show when={props.disconnect !== undefined}>
                <button
                    type="button"
                    class="btn btn-ghost btn-circle tooltip tooltip-bottom tooltip-start"
                    aria-label={t().logout}
                    data-tip={t().logout}
                    onClick={disconnect}
                >
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