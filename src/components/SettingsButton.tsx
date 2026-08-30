import { Settings as SettingsIcon, X } from "lucide-solid";
import { createSignal, Show } from "solid-js";
import { t } from "../i18n";
import { Settings } from "./Settings";

export function SettingsButton({ }) {
    const [open, setOpen] = createSignal(false);

    return (
        <>
            <button
                type="button"
                class="btn btn-ghost btn-circle tooltip tooltip-bottom tooltip-end"
                aria-label={t().settings}
                data-tip={t().settings}
                onClick={() => setOpen(true)}
            >
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
                            class="btn btn-sm btn-circle btn-ghost absolute left-2 top-2 tooltip tooltip-right"
                            aria-label={t().close}
                            data-tip={t().close}
                            onClick={() => setOpen(false)}
                        >
                            <X />
                        </button>
                        <h3 class="font-bold text-lg">{t().settings}</h3>
                        <Settings />
                    </div>
                </div>
            </Show>
        </>
    );
}
