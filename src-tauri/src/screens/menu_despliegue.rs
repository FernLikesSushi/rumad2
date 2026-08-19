//! `MENU DESPLIEGUE:` -- `MENU PRINCIPAL`'s option 5 submenu ("Ver otra
//! informacion"). Structurally identical to `MainMenu` (numbered options,
//! same "0" exit keystroke -- confirmed live, see `RumadScreen::exit`'s
//! doc comment), but kept as its own `TuiScreen` variant rather than
//! reusing `MainMenuScreen` directly, so the frontend can render its own
//! title instead of relabeling it as "MENU PRINCIPAL".

use serde::Serialize;

use super::MenuOption;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct MenuDespliegueScreen {
    pub options: Vec<MenuOption>,
}

#[cfg(test)]
mod tests {
    use crate::screens::{classify, MenuDespliegueScreen, MenuOption, TuiScreen};

    const MENU_DESPLIEGUE: &str = include_str!("../../../screens/menu_despliegue/menu.txt");

    #[test]
    fn classifies_menu_despliegue() {
        let TuiScreen::MenuDespliegue(MenuDespliegueScreen { options }) = classify(MENU_DESPLIEGUE)
        else {
            panic!("expected MenuDespliegue");
        };
        assert_eq!(
            options[0],
            MenuOption {
                key: "1".into(),
                label: "Evaluacion certificacion de Ayuda Economica".into()
            }
        );
        assert_eq!(
            options[4],
            MenuOption {
                key: "5".into(),
                label: "Turno de seleccion de cursos/secciones o Examenes finales".into()
            }
        );
        assert_eq!(
            options[5],
            MenuOption {
                key: "6".into(),
                label: "Horario de cursos disponibles en Matricula".into()
            }
        );
        assert_eq!(
            options.last().unwrap(),
            &MenuOption { key: "0".into(), label: "Finalizar".into() }
        );
    }
}
