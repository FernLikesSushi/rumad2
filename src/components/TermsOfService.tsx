import { X } from "lucide-solid";
import { createSignal, For } from "solid-js";
import { t } from "../i18n";

// Opened from `Settings.tsx` -- same "kept mounted, `.modal-open` toggled"
// pattern as `SettingsButton`'s own nested modal, for the same reason
// (lets daisyUI's open/close transition actually play).
export function TermsOfService() {
    const [open, setOpen] = createSignal(false);

    return (
        <>
            <button type="button" class="link text-sm" onClick={() => setOpen(true)}>
                {t().termsOfService.title}
            </button>

            <div class="modal" classList={{ "modal-open": open() }} onClick={() => setOpen(false)}>
                <div
                    class="modal-box relative max-w-xl"
                    role="dialog"
                    aria-modal="true"
                    onClick={(e) => e.stopPropagation()}
                >
                    <button
                        type="button"
                        class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2"
                        aria-label={t().close}
                        onClick={() => setOpen(false)}
                    >
                        <X />
                    </button>
                    <h3 class="font-bold text-lg mb-2">{t().termsOfService.title}</h3>
                    <div class="flex flex-col gap-3 text-sm max-h-[60vh] overflow-y-auto pr-1">
                        <For each={t().termsOfService.body}>{(paragraph) => <p>{paragraph}</p>}</For>
                    </div>
                    <div class="modal-action">
                        <button class="btn" onClick={() => setOpen(false)}>
                            {t().close}
                        </button>
                    </div>
                </div>
            </div>
        </>
    );
}
