import { Heart } from "lucide-solid";
import { t } from "../i18n";
import { devMode } from "../data/devMode";

export function Footer({ }) {
    // easter egg :)
    const url = devMode() ? import.meta.env.VITE_GITHUB_LINK_DEV : import.meta.env.VITE_GITHUB_LINK_PUBLIC;

    return (
        <footer class="pt-2 sm:pt-4 flex flex-col items-center gap-1 text-xs opacity-60 text-center">
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
