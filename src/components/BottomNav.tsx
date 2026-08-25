import { For } from "solid-js";
import { A } from "@solidjs/router";
import { connected } from "../data/tui";
import { navItems } from "../data/navItems";

// daisyUI's `dock` bottom-navigation bar -- the phone-width nav surface,
// shown only below `sm` (`sm:hidden`). `sm` up to `lg` keeps `Drawer`'s
// hamburger/overlay, `lg`+ keeps its permanent sidebar; all three share
// the same `navItems()` list. `dock` is `position: fixed` at the bottom,
// so callers need their own bottom padding below `sm` to keep content
// from sitting underneath it (see `App.tsx`).
export function BottomNav() {
    return (
        <div class="dock sm:hidden">
            <For each={navItems(connected())}>
                {(item) => (
                    <A href={item.path} end={item.end} activeClass="dock-active">
                        {item.icon}
                        <span class="dock-label">{item.label}</span>
                    </A>
                )}
            </For>
        </div>
    );
}
