import { Heart } from "lucide-solid";
import { t } from "../i18n";
import { devMode } from "../data/devMode";

export function Footer({ }) {
    // easter egg :)
    const url = devMode() ? import.meta.env.VITE_REPO_LINK_DEV : import.meta.env.VITE_REPO_LINK_PUBLIC;

    return (
        <footer class="mt-auto pt-4 flex flex-col items-center gap-1 text-xs opacity-60">
            <div class="flex items-center gap-1">
                {t().madeWith}
                <Heart class="w-6 text-green-300 h-6 fill-current " />
            </div>
            <div>
                <a href={url} target="_blank" rel="noopener noreferrer" class="link">
                    {t().footerBio}
                </a>
            </div>
        </footer>
    );
}
