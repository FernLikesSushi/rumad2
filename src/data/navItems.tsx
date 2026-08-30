import { JSX } from "solid-js";
import { Calendar, Code, Map, PlugZap, Unplug } from "lucide-solid";
import { t } from "../i18n";
import { devMode } from "./devMode";

// Bigger below `sm` (`BottomNav`'s dock) than at `sm`+ (`Drawer`'s
// `is-drawer-close:w-14` collapsed rail, where `size-4` is what fits) --
// same shared list, two different icon sizes per surface.
export const NAV_ICON_CLASS = "my-1.5 inline-block size-6 md:size-4";

export type NavItem = { path: string; end: boolean; label: string; icon: JSX.Element };

// "/" before connecting, "/session" once a TUI session is live -- `connected`
// comes from the Rust backend (`AppState`, the real source of truth), not
// tracked by hand on the frontend. Shared between `Drawer` (sidebar) and
// `BottomNav` (mobile dock) so both nav surfaces list the same items.
export function navItems(connected: boolean): NavItem[] {
    const isConnected = {
        path: "/session",
        end: true,
        label: t().navConnect,
        icon: <Unplug class={NAV_ICON_CLASS} />,
    };
    const isNotConnected = {
        path: "/",
        end: true,
        label: t().navConnect,
        icon: <PlugZap class={NAV_ICON_CLASS} />,
    };

    const list: NavItem[] = [
        connected ? isConnected : isNotConnected,
        { path: "/class-preview", end: true, label: t().navClassEditor, icon: <Calendar class={NAV_ICON_CLASS} /> },
        { path: "/map", end: true, label: t().navMap, icon: <Map class={NAV_ICON_CLASS} /> }
    ];
    if (devMode()) {
        list.push({ path: "/dev", end: false, label: t().devScreens, icon: <Code class={NAV_ICON_CLASS} /> });
    }
    return list;
}
