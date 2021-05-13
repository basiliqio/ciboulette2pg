use super::*;
use std::convert::TryFrom;
use url::Url;

use ciboulette_test_helper::ciboulette::*;
use ciboulette_test_helper::*;

mod delete;
mod inserts;
mod misc;
mod select;
mod update;

pub fn gen_table_store(store: &CibouletteStore) -> Ciboulette2PgTableStore {
    vec![
        Ciboulette2PgTable::new(
            vec![
                Ciboulette2PgId::Uuid(Ciboulette2PgSafeIdent::try_from("people_id").unwrap()),
                Ciboulette2PgId::Uuid(Ciboulette2PgSafeIdent::try_from("article_id").unwrap()),
            ],
            Some(Ciboulette2PgSafeIdent::try_from("public").unwrap()),
            Ciboulette2PgSafeIdent::try_from("people-article").unwrap(),
            store.get_type("people-article").unwrap().clone(),
        ),
        Ciboulette2PgTable::new(
            vec![Ciboulette2PgId::Uuid(
                Ciboulette2PgSafeIdent::try_from("id").unwrap(),
            )],
            Some(Ciboulette2PgSafeIdent::try_from("public").unwrap()),
            Ciboulette2PgSafeIdent::try_from("articles").unwrap(),
            store.get_type("articles").unwrap().clone(),
        ),
        Ciboulette2PgTable::new(
            vec![Ciboulette2PgId::Uuid(
                Ciboulette2PgSafeIdent::try_from("id").unwrap(),
            )],
            Some(Ciboulette2PgSafeIdent::try_from("public").unwrap()),
            Ciboulette2PgSafeIdent::try_from("peoples").unwrap(),
            store.get_type("peoples").unwrap().clone(),
        ),
        Ciboulette2PgTable::new(
            vec![Ciboulette2PgId::Uuid(
                Ciboulette2PgSafeIdent::try_from("id").unwrap(),
            )],
            Some(Ciboulette2PgSafeIdent::try_from("public").unwrap()),
            Ciboulette2PgSafeIdent::try_from("comments").unwrap(),
            store.get_type("comments").unwrap().clone(),
        ),
        Ciboulette2PgTable::new(
            vec![Ciboulette2PgId::Uuid(
                Ciboulette2PgSafeIdent::try_from("id").unwrap(),
            )],
            Some(Ciboulette2PgSafeIdent::try_from("public").unwrap()),
            Ciboulette2PgSafeIdent::try_from("favorite_color").unwrap(),
            store.get_type("favorite_color").unwrap().clone(),
        ),
    ]
    .into_iter()
    .map(|x| (ArcStr::from(x.name().to_string()), Arc::new(x)))
    .collect()
}

impl<'store> std::string::ToString for Ciboulette2PgValue<'store> {
    fn to_string(&self) -> String {
        let null = "<NULL>";
        match self {
            Ciboulette2PgValue::Xml(x) => x
                .clone()
                .map(|x| x.to_string())
                .unwrap_or_else(|| null.to_string()),
            Ciboulette2PgValue::Uuid(x) => x
                .clone()
                .map(|x| x.to_string())
                .unwrap_or_else(|| null.to_string()),
            Ciboulette2PgValue::Time(x) => x
                .clone()
                .map(|x| x.to_string())
                .unwrap_or_else(|| null.to_string()),
            Ciboulette2PgValue::Text(x) => x
                .clone()
                .map(|x| x.to_string())
                .unwrap_or_else(|| null.to_string()),
            Ciboulette2PgValue::ArcStr(x) => x
                .clone()
                .map(|x| x.to_string())
                .unwrap_or_else(|| null.to_string()),
            Ciboulette2PgValue::Numeric(x) => x
                .clone()
                .map(|x| x.to_string())
                .unwrap_or_else(|| null.to_string()),
            Ciboulette2PgValue::Json(x) => x
                .clone()
                .map(|x| x.to_string())
                .unwrap_or_else(|| null.to_string()),
            Ciboulette2PgValue::Integer(x) => x
                .clone()
                .map(|x| x.to_string())
                .unwrap_or_else(|| null.to_string()),
            Ciboulette2PgValue::Float(x) => x
                .clone()
                .map(|x| x.to_string())
                .unwrap_or_else(|| null.to_string()),
            Ciboulette2PgValue::Enum(x) => x
                .clone()
                .map(|x| x.to_string())
                .unwrap_or_else(|| null.to_string()),
            Ciboulette2PgValue::Double(x) => x
                .clone()
                .map(|x| x.to_string())
                .unwrap_or_else(|| null.to_string()),
            Ciboulette2PgValue::DateTime(x) => x
                .clone()
                .map(|x| x.to_string())
                .unwrap_or_else(|| null.to_string()),
            Ciboulette2PgValue::Date(x) => x
                .clone()
                .map(|x| x.to_string())
                .unwrap_or_else(|| null.to_string()),
            Ciboulette2PgValue::Char(x) => x
                .clone()
                .map(|x| x.to_string())
                .unwrap_or_else(|| null.to_string()),
            Ciboulette2PgValue::Bytes(x) => x
                .clone()
                .map(|x| String::from_utf8_lossy(x.as_ref()).to_string())
                .unwrap_or_else(|| null.to_string()),
            Ciboulette2PgValue::Boolean(x) => x
                .clone()
                .map(|x| x.to_string())
                .unwrap_or_else(|| null.to_string()),
            Ciboulette2PgValue::Array(x) => x
                .clone()
                .map(|x| x.into_iter().map(|y| y.to_string()).collect())
                .unwrap_or_else(|| null.to_string()),
        }
    }
}

impl<'store> std::string::ToString for Ciboulette2PgArguments<'store> {
    fn to_string(&self) -> String {
        self.inner.iter().cloned().map(|x| x.to_string()).collect()
    }
}

#[macro_export]
macro_rules! test_sql {
    ($query:ident) => {
        let (query, _) = $query;
        // let stringified_params: Vec<String> = params.iter().map(|x| x.to_string()).collect();

        insta::assert_snapshot!(sqlformat::format(
            query.as_str(),
            &sqlformat::QueryParams::None,
            sqlformat::FormatOptions::default()
        ));
    };
}
