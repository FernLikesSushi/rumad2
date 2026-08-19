//! "Indique Semestre:" -- which term to browse sections for, shown after a
//! successful `Login`. No dedicated types of its own: it reuses
//! `MenuOption` and `scrape::scrape_equals_options`.

#[cfg(test)]
mod tests {
    use crate::screens::{classify, MenuOption, TuiScreen};

    const SELECT_PERIOD: &str = include_str!("../../../screens/matricula/select_period.txt");

    #[test]
    fn classifies_select_period() {
        let TuiScreen::SelectPeriod { options } = classify(SELECT_PERIOD) else {
            panic!("expected SelectPeriod");
        };
        assert_eq!(
            options,
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
}
