import { Settings as SettingsIcon, X } from "lucide-solid";
import { createSignal, Show } from "solid-js";
import { t } from "../i18n";
import { Settings } from "./Settings";

export function SettingsButton({ }) {
    // TODO: localize the aria-label/title text below
    const [open, setOpen] = createSignal(false);

    return (
        <>
            <button type="button" class="btn btn-ghost btn-circle" aria-label={"Settings"} onClick={() => setOpen(true)}>
                <SettingsIcon />
            </button>

            <Show when={open()}>
                <div class="modal modal-open" onClick={() => setOpen(false)}>
                    <div
                        class="modal-box relative max-w-xl px-4 py-16"
                        role="dialog"
                        aria-modal="true"
                        onClick={(e) => e.stopPropagation()}
                    >
                        <button
                            type="button"
                            class="btn btn-sm btn-circle btn-ghost absolute left-2 top-2"
                            aria-label={t().close}
                            onClick={() => setOpen(false)}
                        >
                            <X />
                        </button>
                        <h3 class="font-bold text-lg">{"Settings"}</h3>
                        <Settings />
                    </div>
                </div>
            </Show>
        </>
    );
}
