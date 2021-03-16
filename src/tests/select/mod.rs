use super::*;

mod select_success;

pub fn check_rows(rows: &Vec<Ciboulette2PostgresRow>) {
    insta::assert_json_snapshot!(rows,
    {
        ".*.id" => insta::dynamic_redaction(|value, _path| {
            assert_eq!(value
                .as_str()
                .unwrap()
                .chars()
                .filter(|&c| c == '-')
                .count(),
                4
            );
            "[uuid]"
        })
    });
}
