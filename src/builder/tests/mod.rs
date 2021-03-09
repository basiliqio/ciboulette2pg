use super::*;
use std::convert::TryFrom;
use url::Url;

use ciboulette_test_helper::ciboulette::*;
use ciboulette_test_helper::*;

mod delete;
mod inserts;
mod misc;
mod update;

fn gen_table_store<'a>() -> Ciboulette2PostgresTableStore<'a> {
    vec![
        Ciboulette2PostgresTableSettings::new(
            Ciboulette2PostgresSafeIdent::try_from("id").unwrap(),
            Ciboulette2PostgresSafeIdent::try_from("uuid").unwrap(),
            Some(Ciboulette2PostgresSafeIdent::try_from("public").unwrap()),
            Ciboulette2PostgresSafeIdent::try_from("people-article").unwrap(),
        ),
        Ciboulette2PostgresTableSettings::new(
            Ciboulette2PostgresSafeIdent::try_from("id").unwrap(),
            Ciboulette2PostgresSafeIdent::try_from("uuid").unwrap(),
            Some(Ciboulette2PostgresSafeIdent::try_from("public").unwrap()),
            Ciboulette2PostgresSafeIdent::try_from("articles").unwrap(),
        ),
        Ciboulette2PostgresTableSettings::new(
            Ciboulette2PostgresSafeIdent::try_from("id").unwrap(),
            Ciboulette2PostgresSafeIdent::try_from("uuid").unwrap(),
            Some(Ciboulette2PostgresSafeIdent::try_from("public").unwrap()),
            Ciboulette2PostgresSafeIdent::try_from("peoples").unwrap(),
        ),
        Ciboulette2PostgresTableSettings::new(
            Ciboulette2PostgresSafeIdent::try_from("id").unwrap(),
            Ciboulette2PostgresSafeIdent::try_from("uuid").unwrap(),
            Some(Ciboulette2PostgresSafeIdent::try_from("public").unwrap()),
            Ciboulette2PostgresSafeIdent::try_from("favorite_color").unwrap(),
        ),
    ]
    .into_iter()
    .map(|x| (x.name().to_string(), x))
    .collect()
}
