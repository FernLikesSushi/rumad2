import type { JSX } from "solid-js";
import { LogOut, Plus, Minus, RefreshCw, Calendar } from "lucide-solid";
import type { MenuKind } from "./types";

export type IconMenu = MenuKind | "Matricula" | "Unknown";

// Matricula's Actions prompt keys (A=Alta B=Baja C=Cambio ... S=Salir --
// see `Messages.actionLabels`' own doc comment for the full list).
const MATRICULA_ICONS: Record<string, () => JSX.Element> = {
  // add
  A: () => <Plus class="w-4 h-4" />,
  // remove
  B: () => <Minus class="w-4 h-4" />,
  // change
  C: () => <RefreshCw class="w-4 h-4" />,
  // course schedule
  H: () => <Calendar class="w-4 h-4" />,
  // exit
  S: () => <LogOut class="w-4 h-4" />,
};

export function iconFor(menu: IconMenu, key: string): JSX.Element | null {
  if ((menu === "MainMenu" || menu === "MenuDespliegue") && key === "0") return <LogOut class="w-4 h-4" />;
  if (menu === "Matricula") return MATRICULA_ICONS[key]?.() ?? null;
  return null;
}
