import { PawPrint } from "lucide-solid";
import { createEffect, createSignal, Signal } from "solid-js";
import { locale, setLocale, t } from "../i18n";
import { loadUsername, password, saveUsername, setPassword } from "../username";
import { devMode, setDevMode } from "../devMode";
import { Toggle } from "./Toggle";
import { ToggleButton } from "./ToggleButton";

export function Settings() {

    const [username, setUsername] = createSignal(loadUsername());


    createEffect(() => {
        saveUsername(username());
    });
    
    function submit(e: Event) {
        e.preventDefault();
    }

    return <form class="flex flex-col items-center gap-2.5 max-w-lg mx-auto text-center" onSubmit={submit}>
        <p class="text-[0.85em] opacity-75">{t().loginHint}</p>
        <input
            class="input w-full max-w-xs"
            placeholder={t().usernamePlaceholder}
            value={username()}
            onInput={(e) => setUsername(e.currentTarget.value)}
        />
        <input
            type="password"
            class="input w-full max-w-xs"
            placeholder={t().passwordPlaceholder}
            value={password()}
            onInput={(e) => setPassword(e.currentTarget.value)}
        />

        <div class="divider w-full" />

        <div class="flex items-center gap-2">
            <ToggleButton label="🇵🇷 Español" active={locale() === "es"} onClick={() => setLocale("es")} />
            <ToggleButton label="🇬🇧 English" active={locale() === "en"} onClick={() => setLocale("en")} />
        </div>

        <div class="divider w-full" />

        <Toggle label={t().developerMode} checked={devMode()} onChange={setDevMode} />
    </form>
}