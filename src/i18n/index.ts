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

// Menu-driven screens (Matricula's Actions, MenuScreen's every MenuKind)
// render buttons labeled with the remote's own raw text -- localize by
// string-matching against whichever catalog covers that label
// (`actionLabels`/`periodLabels`/`menuLabels`; their key sets don't
// overlap), falling back to the raw label for anything none of them list.
export function localizeButton(label: string): string {
  const messages = t();
  return messages.actionLabels[label] ?? messages.periodLabels[label] ?? messages.menuLabels[label] ?? label;
}
