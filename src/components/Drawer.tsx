import { For, JSX } from "solid-js";
import { A } from "@solidjs/router";
import { Calendar, Code, Map, Menu, PlugZap, Unplug } from "lucide-solid";
import { t } from "../i18n";
import { devMode } from "../data/devMode";
import { connected } from "../data/tui";

const DRAWER_ID = "app-drawer";
const ICON_CLASS = "my-1.5 inline-block size-4";

type NavItem = { path: string; end: boolean; label: string; icon: JSX.Element };

// "/" before connecting, "/session" once a TUI session is live -- `connected`
// comes from the Rust backend (`AppState`, the real source of truth), not
// tracked by hand on the frontend.
function items(connected: boolean): NavItem[] {
    const isConnected = {
        path: "/session",
        end: true,
        label: t().navConnect,
        icon: <Unplug class={ICON_CLASS} />,
    };
    const isNotConnected = {
        path: "/",
        end: true,
        label: t().navConnect,
        icon: <PlugZap class={ICON_CLASS} />,
    };

    const list: NavItem[] = [
        connected ? isConnected : isNotConnected,
        { path: "/class-preview", end: true, label: t().navClassEditor, icon: <Calendar class={ICON_CLASS} /> },
        { path: "/map", end: true, label: t().navMap, icon: <Map class={ICON_CLASS} /> }
    ];
    if (devMode()) {
        list.push({ path: "/dev", end: false, label: t().devScreens, icon: <Code class={ICON_CLASS} /> });
    }
    return list;
}

// daisyUI's collapsible icon-only
// drawer pattern: permanently open as a sidebar on large screens
// (`lg:drawer-open`), a toggleable overlay below that. Pure CSS via the
// `is-drawer-open`/`is-drawer-close` variants driven by the checkbox --
// no signal needed. https://daisyui.com/components/drawer/#responsive-collapsible-icon-only-drawer-sidebar-using-is-drawer-close-and-is-drawer-open
export function Drawer(props: { children: JSX.Element }) {
    return (
        <div class="drawer lg:drawer-open">
            <input id={DRAWER_ID} type="checkbox" class="drawer-toggle" />
            <div class="drawer-content">
                <div class="p-2 lg:hidden">
                    <label for={DRAWER_ID} aria-label={t().openMenu} class="btn btn-square btn-ghost drawer-button">
                        <Menu class={ICON_CLASS} />
                    </label>
                </div>
                {props.children}
            </div>

            <div class="drawer-side is-drawer-close:overflow-visible">
                <label for={DRAWER_ID} aria-label={t().closeMenu} class="drawer-overlay" />
                <div class="flex min-h-full flex-col items-start bg-base-200 is-drawer-close:w-14 is-drawer-open:w-64">
                    <ul class="menu w-full grow">
                        <For each={items(connected())}>
                            {(item) => (
                                <li>
                                    <A
                                        href={item.path}
                                        end={item.end}
                                        class="is-drawer-close:tooltip is-drawer-close:tooltip-right"
                                        data-tip={item.label}
                                    >
                                        {item.icon}
                                        <span class="is-drawer-close:hidden">{item.label}</span>
                                    </A>
                                </li>
                            )}
                        </For>
                    </ul>
                </div>
            </div>
        </div>
    );
}
