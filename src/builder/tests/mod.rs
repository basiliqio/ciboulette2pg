use super::*;
use std::convert::TryFrom;
use url::Url;

use ciboulette_test_helper::ciboulette::*;
use ciboulette_test_helper::*;

mod delete;
mod inserts;
mod misc;
mod update;

fn gen_table_store<'a>(store: &'a CibouletteStore<'a>) -> Ciboulette2PostgresTableStore<'a> {
    vec![
        Ciboulette2PostgresTableSettings::new(
            Ciboulette2PostgresSafeIdent::try_from("id").unwrap(),
            Ciboulette2PostgresSafeIdent::try_from("uuid").unwrap(),
            Some(Ciboulette2PostgresSafeIdent::try_from("public").unwrap()),
            Ciboulette2PostgresSafeIdent::try_from("people-article").unwrap(),
            store.get_type("people-article").unwrap(),
        ),
        Ciboulette2PostgresTableSettings::new(
            Ciboulette2PostgresSafeIdent::try_from("id").unwrap(),
            Ciboulette2PostgresSafeIdent::try_from("uuid").unwrap(),
            Some(Ciboulette2PostgresSafeIdent::try_from("public").unwrap()),
            Ciboulette2PostgresSafeIdent::try_from("articles").unwrap(),
            store.get_type("articles").unwrap(),
        ),
        Ciboulette2PostgresTableSettings::new(
            Ciboulette2PostgresSafeIdent::try_from("id").unwrap(),
            Ciboulette2PostgresSafeIdent::try_from("uuid").unwrap(),
            Some(Ciboulette2PostgresSafeIdent::try_from("public").unwrap()),
            Ciboulette2PostgresSafeIdent::try_from("peoples").unwrap(),
            store.get_type("peoples").unwrap(),
        ),
        Ciboulette2PostgresTableSettings::new(
            Ciboulette2PostgresSafeIdent::try_from("id").unwrap(),
            Ciboulette2PostgresSafeIdent::try_from("uuid").unwrap(),
            Some(Ciboulette2PostgresSafeIdent::try_from("public").unwrap()),
            Ciboulette2PostgresSafeIdent::try_from("favorite_color").unwrap(),
            store.get_type("favorite_color").unwrap(),
        ),
    ]
    .into_iter()
    .map(|x| (x.name().to_string(), x))
    .collect()
}

impl<'a> std::string::ToString for Ciboulette2SqlValue<'a> {
    fn to_string(&self) -> String {
        let null = "<NULL>";
        match self {
            Ciboulette2SqlValue::Xml(x) => x
                .clone()
                .map(|x| x.to_string())
                .unwrap_or_else(|| null.to_string()),
            Ciboulette2SqlValue::Uuid(x) => x
                .clone()
                .map(|x| x.to_string())
                .unwrap_or_else(|| null.to_string()),
            Ciboulette2SqlValue::Time(x) => x
                .clone()
                .map(|x| x.to_string())
                .unwrap_or_else(|| null.to_string()),
            Ciboulette2SqlValue::Text(x) => x
                .clone()
                .map(|x| x.to_string())
                .unwrap_or_else(|| null.to_string()),
            Ciboulette2SqlValue::Numeric(x) => x
                .clone()
                .map(|x| x.to_string())
                .unwrap_or_else(|| null.to_string()),
            Ciboulette2SqlValue::Json(x) => x
                .clone()
                .map(|x| x.to_string())
                .unwrap_or_else(|| null.to_string()),
            Ciboulette2SqlValue::Integer(x) => x
                .clone()
                .map(|x| x.to_string())
                .unwrap_or_else(|| null.to_string()),
            Ciboulette2SqlValue::Float(x) => x
                .clone()
                .map(|x| x.to_string())
                .unwrap_or_else(|| null.to_string()),
            Ciboulette2SqlValue::Enum(x) => x
                .clone()
                .map(|x| x.to_string())
                .unwrap_or_else(|| null.to_string()),
            Ciboulette2SqlValue::Double(x) => x
                .clone()
                .map(|x| x.to_string())
                .unwrap_or_else(|| null.to_string()),
            Ciboulette2SqlValue::DateTime(x) => x
                .clone()
                .map(|x| x.to_string())
                .unwrap_or_else(|| null.to_string()),
            Ciboulette2SqlValue::Date(x) => x
                .clone()
                .map(|x| x.to_string())
                .unwrap_or_else(|| null.to_string()),
            Ciboulette2SqlValue::Char(x) => x
                .clone()
                .map(|x| x.to_string())
                .unwrap_or_else(|| null.to_string()),
            Ciboulette2SqlValue::Bytes(x) => x
                .clone()
                .map(|x| String::from_utf8_lossy(x.as_ref()).to_string())
                .unwrap_or_else(|| null.to_string()),
            Ciboulette2SqlValue::Boolean(x) => x
                .clone()
                .map(|x| x.to_string())
                .unwrap_or_else(|| null.to_string()),
            Ciboulette2SqlValue::Array(x) => x
                .clone()
                .map(|x| x.into_iter().map(|y| y.to_string()).collect())
                .unwrap_or_else(|| null.to_string()),
        }
    }
}

impl<'a> std::string::ToString for Ciboulette2SqlArguments<'a> {
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
