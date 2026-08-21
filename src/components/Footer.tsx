import { Heart } from "lucide-solid";
import { t } from "../i18n";
import { devMode } from "../devMode";

export function Footer({ }) {
    // easter egg :)
    const url = devMode() ? import.meta.env.VITE_GITHUB_LINK_DEV : import.meta.env.VITE_GITHUB_LINK_PUBLIC;

    return (
        <footer class="mt-auto pt-4 flex flex-col items-center gap-1 text-xs opacity-60">
            <div class="flex items-center gap-1">
                {t().madeWith}
                <Heart class="w-6 text-green-300 h-6 fill-current " />
            </div>
            <div>
                By a{" "}
                <a href={url} target="_blank" rel="noopener noreferrer" class="link">
                    software engineer who loves sushi, cake and exploding kittens.
                </a>
            </div>
        </footer>
    );
}
