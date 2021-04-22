use super::*;
use std::convert::TryFrom;

/// Type of additional field
#[derive(Clone, Debug)]
pub enum Ciboulette2SqlAdditionalFieldType {
    /// A field required to link a relationship
    Relationship,
    /// A field required for sorting
    Sorting,
}

impl Ciboulette2SqlAdditionalFieldType {
    /// Print the prefix of the additional type to str
    pub(crate) fn to_safe_modifier(&self) -> Ciboulette2PostgresSafeIdentModifier {
        match self {
            Ciboulette2SqlAdditionalFieldType::Relationship => {
                Ciboulette2PostgresSafeIdentModifier::Prefix(CIBOULETTE_REL_PREFIX)
            }
            Ciboulette2SqlAdditionalFieldType::Sorting => {
                Ciboulette2PostgresSafeIdentModifier::Prefix(CIBOULETTE_SORT_PREFIX)
            }
        }
    }
}

#[derive(Clone, Debug, Getters)]
#[getset(get = "pub")]
pub(crate) struct Ciboulette2SqlAdditionalField {
    pub(crate) type_: Ciboulette2SqlAdditionalFieldType,
    pub(crate) ident: Ciboulette2PostgresTableField,
    pub(crate) name: Ciboulette2PostgresSafeIdent,
    pub(crate) ciboulette_type: Arc<CibouletteResourceType>,
}

impl Ciboulette2SqlAdditionalField {
    pub fn new(
        ident: Ciboulette2PostgresTableField,
        type_: Ciboulette2SqlAdditionalFieldType,
        ciboulette_type: Arc<CibouletteResourceType>,
    ) -> Self {
        Ciboulette2SqlAdditionalField {
            name: ident.name().clone().add_modifier(type_.to_safe_modifier()),
            ident,
            type_,
            ciboulette_type,
        }
    }

    pub fn from_sorting_field(
        el: &CibouletteSortingElement,
        main_type: Arc<CibouletteResourceType>,
    ) -> Result<Self, Ciboulette2SqlError> {
        let table_field = Ciboulette2PostgresTableField::new(
            Ciboulette2PostgresSafeIdent::try_from(el.field().clone())?,
            None,
            None,
        );
        Ok(Ciboulette2SqlAdditionalField::new(
            table_field,
            Ciboulette2SqlAdditionalFieldType::Sorting,
            main_type,
        ))
    }
}
