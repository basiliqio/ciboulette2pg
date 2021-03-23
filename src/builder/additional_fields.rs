use super::*;
use std::convert::TryFrom;

/// Type of additional field
#[derive(Clone, Debug)]
pub enum Ciboulette2SqlAdditionalFieldType {
    /// A field required to link a relationship
    Relationship,
    /// A field required to link compare with another identifier of the same type
    MainIdentifier,
    /// A field required for sorting
    Sorting,
}

impl Ciboulette2SqlAdditionalFieldType {
    /// Print the prefix of the additional type to str
    pub(crate) fn to_sql_prefix(&self) -> &str {
        match self {
            Ciboulette2SqlAdditionalFieldType::Relationship => "rel",
            Ciboulette2SqlAdditionalFieldType::Sorting => "sort",
            Ciboulette2SqlAdditionalFieldType::MainIdentifier => "",
        }
    }
}

#[derive(Clone, Debug, Getters)]
#[getset(get = "pub")]
pub(crate) struct Ciboulette2SqlAdditionalField<'a> {
    pub(crate) type_: Ciboulette2SqlAdditionalFieldType,
    pub(crate) ident: Ciboulette2PostgresTableField<'a>,
    pub(crate) name: Ciboulette2PostgresSafeIdent<'a>,
    pub(crate) ciboulette_type: &'a CibouletteResourceType<'a>,
}

impl<'a> Ciboulette2SqlAdditionalField<'a> {
    pub fn new(
        ident: Ciboulette2PostgresTableField<'a>,
        type_: Ciboulette2SqlAdditionalFieldType,
        ciboulette_type: &'a CibouletteResourceType<'a>,
    ) -> Result<Self, Ciboulette2SqlError> {
        Ok(Ciboulette2SqlAdditionalField {
            name: Ciboulette2PostgresSafeIdent::try_from(format!(
                "{}_{}",
                type_.to_sql_prefix(),
                ident.name()
            ))?,
            ident,
            type_,
            ciboulette_type,
        })
    }
}

impl<'a> TryFrom<&Ciboulette2PostgresTable<'a>> for Ciboulette2SqlAdditionalField<'a> {
    type Error = Ciboulette2SqlError;

    fn try_from(table: &Ciboulette2PostgresTable<'a>) -> Result<Self, Self::Error> {
        Ciboulette2SqlAdditionalField::new(
            Ciboulette2PostgresTableField::from(table.id()),
            Ciboulette2SqlAdditionalFieldType::MainIdentifier,
            table.ciboulette_type(),
        )
    }
}

impl<'a> TryFrom<&CibouletteSortingElement<'a>> for Ciboulette2SqlAdditionalField<'a> {
    type Error = Ciboulette2SqlError;

    fn try_from(el: &CibouletteSortingElement<'a>) -> Result<Self, Self::Error> {
        let table_field = Ciboulette2PostgresTableField::new_owned(
            Ciboulette2PostgresSafeIdent::try_from(el.field().clone())?,
            None,
            None,
        );
        Ciboulette2SqlAdditionalField::new(
            table_field,
            Ciboulette2SqlAdditionalFieldType::Sorting,
            el.type_(),
        )
    }
}
