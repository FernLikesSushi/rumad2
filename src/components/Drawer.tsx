import { For, JSX } from "solid-js";
import { A } from "@solidjs/router";
import { Menu } from "lucide-solid";
import { t } from "../i18n";
import { connected } from "../data/tui";
import { navItems } from "../data/navItems";

const DRAWER_ID = "app-drawer";
const ICON_CLASS = "my-1.5 inline-block size-4";

// daisyUI's collapsible icon-only
// drawer pattern: permanently open as a sidebar on large screens
// (`lg:drawer-open`), a toggleable overlay between `sm` and `lg`. Pure CSS
// via the `is-drawer-open`/`is-drawer-close` variants driven by the
// checkbox -- no signal needed.
// https://daisyui.com/components/drawer/#responsive-collapsible-icon-only-drawer-sidebar-using-is-drawer-close-and-is-drawer-open
// Below `sm` there's no way to reach the hamburger (hidden, see below), so
// `BottomNav`'s dock takes over as the phone-width nav surface instead --
// `max-sm:hidden` on `drawer-side` keeps the overlay panel from ever being
// shown there even if the checkbox state carries over from a resize.
export function Drawer(props: { children: JSX.Element }) {
    return (
        <div class="drawer lg:drawer-open">
            <input id={DRAWER_ID} type="checkbox" class="drawer-toggle" />
            <div class="drawer-content">
                <div class="p-2 hidden sm:block lg:hidden">
                    <label for={DRAWER_ID} aria-label={t().openMenu} class="btn btn-square btn-ghost drawer-button">
                        <Menu class={ICON_CLASS} />
                    </label>
                </div>
                {props.children}
            </div>

            <div class="drawer-side is-drawer-close:overflow-visible max-sm:hidden">
                <label for={DRAWER_ID} aria-label={t().closeMenu} class="drawer-overlay" />
                <div class="flex min-h-full flex-col items-start bg-base-200 is-drawer-close:w-14 is-drawer-open:w-64">
                    <ul class="menu w-full grow">
                        <For each={navItems(connected())}>
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
