import { createMemo, createSignal } from "solid-js";
import type { Messages } from "./types";
import es from "./es";
import en from "./en";

export type Locale = "es" | "en";

const catalogs: Record<Locale, Messages> = { es, en };

function detectLocale(): Locale {
  const lang = typeof navigator !== "undefined" ? navigator.language : "es";
  return lang.toLowerCase().startsWith("en") ? "en" : "es";
}

export const [locale, setLocale] = createSignal<Locale>(detectLocale());

/** Reactive accessor for the current locale's message catalog: `t().send`. */
export const t = createMemo(() => catalogs[locale()]);
