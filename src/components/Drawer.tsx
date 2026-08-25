import { For, JSX } from "solid-js";
import { A } from "@solidjs/router";
import { t } from "../i18n";
import { connected } from "../data/tui";
import { navItems } from "../data/navItems";

// Exported so `Header` can render the toggle label itself (see there for
// why) instead of this owning a separate one.
export const DRAWER_ID = "app-drawer";

// daisyUI's collapsible icon-only
// drawer pattern: permanently open as a sidebar on large screens
// (`lg:drawer-open`), a toggleable overlay between `sm` and `lg`. Pure CSS
// via the `is-drawer-open`/`is-drawer-close` variants driven by the
// checkbox -- no signal needed.
// https://daisyui.com/components/drawer/#responsive-collapsible-icon-only-drawer-sidebar-using-is-drawer-close-and-is-drawer-open
// Below `sm` there's no way to reach the hamburger (hidden, see `Header`),
// so `BottomNav`'s dock takes over as the phone-width nav surface instead --
// `max-sm:hidden` on `drawer-side` keeps the overlay panel from ever being
// shown there even if the checkbox state carries over from a resize.
export function Drawer(props: { children: JSX.Element }) {
    return (
        // `flex-1`, not a height of its own -- this fills whatever `main`
        // (its parent, see `App.tsx`) actually gives it rather than
        // independently asserting a viewport-relative height that could
        // stack with `main`'s and overflow it.
        <div class="drawer lg:drawer-open flex-1">
            <input id={DRAWER_ID} type="checkbox" class="drawer-toggle" />
            {/* `flex flex-col h-full` -- daisyUI's own `.drawer-content`
                rule is a plain grid item (no `display:flex`), so without
                this, `{props.children}` (each page's own `main`-level
                content, e.g. `flex-1`/`mt-auto` children) never becomes a
                real flex item with real leftover space to grow into or
                push against; it just sits in normal block flow instead. */}
            <div class="drawer-content flex flex-col h-full">
                {props.children}
            </div>

            {/* `h-full`, overriding daisyUI's own `.drawer-side` rule
                (`height: 100dvh`, unconditional) -- that hardcodes a second
                independent viewport-relative height inside here regardless
                of this element's actual grid cell size, which is exactly
                the kind of stacking `flex-1` above is meant to avoid. */}
            <div class="drawer-side is-drawer-close:overflow-visible max-sm:hidden h-full">
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
