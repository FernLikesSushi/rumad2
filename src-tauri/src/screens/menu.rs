//! `MenuScreen` -- the shape shared by every screen that's just a
//! numbered/lettered list of options read one keystroke at a time. Four
//! real remote prompts render this way; rather than four near-identical
//! structs, `MenuKind` says which one a given `MenuScreen` is (the
//! frontend picks its own title/behavior off it) while the struct and its
//! `RumadScreen` impl stay shared. Three of the four are fully identical
//! in behavior too (default `select`/`line`/`exit`, exit key "0");
//! `HorarioSemester` is the one exception (no free-text prompt at all, and
//! exits via PF4 instead of "0"), so `RumadScreen for MenuScreen` branches
//! on `kind` for just those two methods.

use regex::Regex;
use serde::Serialize;
use std::sync::OnceLock;

use super::MenuOption;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum MenuKind {
    /// `MENU PRINCIPAL:` -- the very first screen after connecting, e.g.
    /// "1.  Seleccion de Secciones  (Matricula)" ... "0.  SALIR DEL
    /// SISTEMA".
    MainMenu,
    /// `MENU PRINCIPAL`'s option 5 submenu ("Ver otra informacion"), e.g.
    /// "1.  Evaluacion certificacion de Ayuda Economica" ... "0.
    /// Finalizar". Structurally identical to `MainMenu` -- confirmed live,
    /// its own "0. Finalizar" is the same exit key `MainMenu`'s "0. SALIR
    /// DEL SISTEMA" uses -- but kept as its own `MenuKind` so the frontend
    /// renders its own title instead of relabeling it as "MENU PRINCIPAL".
    MenuDespliegue,
    /// `"Indique Semestre:"` -- which term to browse sections for, shown
    /// after a successful `Login`, e.g. "1=1er Sem   2=2do Sem   3=1er
    /// Verano o Verano Extendido   4=2do Verano o Admision Temprana
    /// S=salir".
    SelectPeriod,
    /// `MenuDespliegue` option 6's first step ("Horario de cursos
    /// disponibles en Matricula"): pick a semester before searching a
    /// course code, e.g. "Indique semestre  (1=1erVer   2=1erSem
    /// 3=2doSem   4=2doVer)      [PF4=(9)Fin]". Distinct from
    /// `SelectPeriod`'s own "Indique Semestre:" (capital S) in the
    /// `Login`/`Matricula` flow -- this one is lowercase, compact
    /// ("1=1erVer" rather than "1=1er Sem"), parenthesized, and exits via
    /// `[PF4=(9)Fin]` instead of a listed "S=salir" option (see
    /// `screens::tests::horario_semestre_is_not_mistaken_for_select_period`).
    HorarioSemester,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct MenuScreen {
    pub menu: MenuKind,
    pub options: Vec<MenuOption>,
}

/// Unique to `HorarioSemester`'s prompt: lowercase "semestre", unlike
/// `SelectPeriod`'s "Indique Semestre:".
pub(super) const HORARIO_SEMESTER_DETECT_HINT: &str = "Indique semestre";

fn compact_equals_pattern() -> &'static Regex {
    static PATTERN: OnceLock<Regex> = OnceLock::new();
    // "1=1erVer   2=1erSem   3=2doSem   4=2doVer" -- no spaces around "=".
    // `scrape::scrape_equals_options`'s generic word-splitting would
    // misfire here: the enclosing parens glue onto the adjacent
    // digit/word ("(1=1erVer", "4=2doVer)"), and the line's own
    // "[PF4=(9)Fin]" hint isn't a real option. Anchoring the key to `\d+`
    // specifically (PF4's key is letters) and the label to `\w+` (which
    // stops at the ")" following the last option, and can never start a
    // match right after "PF4=" since a "(" immediately follows there)
    // sidesteps both without needing to special-case the parens at all.
    PATTERN.get_or_init(|| Regex::new(r"(\d+)=(\w+)").unwrap())
}

pub(super) fn scrape_horario_semester_options(raw: &str) -> Vec<MenuOption> {
    compact_equals_pattern()
        .captures_iter(raw)
        .map(|c| MenuOption { key: c[1].to_string(), label: c[2].to_string() })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::screens::{classify, MenuKind, MenuOption, TuiScreen};

    const MENU_DESPLIEGUE: &str = include_str!("../../../screens/menu_despliegue/menu.txt");
    const SELECT_PERIOD: &str = include_str!("../../../screens/matricula/select_period.txt");
    const HORARIO_SEMESTRE: &str =
        include_str!("../../../screens/menu_despliegue/horario_semestre.txt");

    #[test]
    fn classifies_menu_despliegue() {
        let TuiScreen::Menu(screen) = classify(MENU_DESPLIEGUE).screen else {
            panic!("expected Menu");
        };
        assert_eq!(screen.menu, MenuKind::MenuDespliegue);
        assert_eq!(
            screen.options[0],
            MenuOption {
                key: "1".into(),
                label: "Evaluacion certificacion de Ayuda Economica".into()
            }
        );
        assert_eq!(
            screen.options[5],
            MenuOption {
                key: "6".into(),
                label: "Horario de cursos disponibles en Matricula".into()
            }
        );
        assert_eq!(
            screen.options.last().unwrap(),
            &MenuOption { key: "0".into(), label: "Finalizar".into() }
        );
    }

    #[test]
    fn classifies_select_period() {
        let TuiScreen::Menu(screen) = classify(SELECT_PERIOD).screen else {
            panic!("expected Menu");
        };
        assert_eq!(screen.menu, MenuKind::SelectPeriod);
        assert_eq!(
            screen.options,
            vec![
                MenuOption { key: "1".into(), label: "1er Sem".into() },
                MenuOption { key: "2".into(), label: "2do Sem".into() },
                MenuOption {
                    key: "3".into(),
                    label: "1er Verano o Verano Extendido".into()
                },
                MenuOption {
                    key: "4".into(),
                    label: "2do Verano o Admision Temprana".into()
                },
                MenuOption { key: "S".into(), label: "salir".into() },
            ]
        );
    }

    #[test]
    fn classifies_horario_semester() {
        let TuiScreen::Menu(screen) = classify(HORARIO_SEMESTRE).screen else {
            panic!("expected Menu");
        };
        assert_eq!(screen.menu, MenuKind::HorarioSemester);
        assert_eq!(
            screen.options,
            vec![
                MenuOption { key: "1".into(), label: "1erVer".into() },
                MenuOption { key: "2".into(), label: "1erSem".into() },
                MenuOption { key: "3".into(), label: "2doSem".into() },
                MenuOption { key: "4".into(), label: "2doVer".into() },
            ]
        );
    }

    // `Indique semestre` here is lowercase and shaped differently
    // ("(1=1erVer   2=1erSem ...)" plus a `[PF4=(9)Fin]` hint) from the
    // Matricula flow's own period prompt (`Indique Semestre:`, capital
    // S) -- a different real screen that happens to share a similar
    // phrase, not the same prompt.
    #[test]
    fn horario_semestre_is_not_mistaken_for_select_period() {
        let TuiScreen::Menu(screen) = classify(HORARIO_SEMESTRE).screen else {
            panic!("expected Menu");
        };
        assert_ne!(screen.menu, MenuKind::SelectPeriod);
    }
}
