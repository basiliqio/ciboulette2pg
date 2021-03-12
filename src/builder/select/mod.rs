use super::*;

pub mod json;
pub mod main;
pub mod rel;
pub mod related;
pub mod utils;

const EMPTY_LIST: [Cow<'static, str>; 0] = [];

#[derive(Clone, Debug)]
pub enum Ciboulette2SqlAdditonalFieldType {
    Sort,
    Relationship,
}

impl Ciboulette2SqlAdditonalFieldType {
    pub fn as_str(&self) -> &str {
        match self {
            Ciboulette2SqlAdditonalFieldType::Sort => "sort",
            Ciboulette2SqlAdditonalFieldType::Relationship => "sort",
        }
    }
}

#[derive(Clone, Debug, Getters)]
#[getset(get = "pub")]
pub struct Ciboulette2SqlAdditonalField<'a> {
    type_: Ciboulette2SqlAdditonalFieldType,
    ident: Ciboulette2PostgresTableField<'a>,
}

impl<'a> Ciboulette2SqlAdditonalField<'a> {
    pub fn new(
        ident: Ciboulette2PostgresTableField<'a>,
        type_: Ciboulette2SqlAdditonalFieldType,
    ) -> Self {
        Ciboulette2SqlAdditonalField { ident, type_ }
    }
}
