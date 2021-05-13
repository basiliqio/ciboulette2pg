use super::*;
use std::convert::TryFrom;

/// Type of additional field
#[derive(Clone, Debug)]
pub enum Ciboulette2PgAdditionalFieldType {
    /// A field required to link a relationship
    Relationship,
    /// A field required for sorting
    Sorting,
}

impl Ciboulette2PgAdditionalFieldType {
    /// Print the prefix of the additional type to str
    pub(crate) fn to_safe_modifier(&self) -> Ciboulette2PgSafeIdentModifier {
        match self {
            Ciboulette2PgAdditionalFieldType::Relationship => {
                Ciboulette2PgSafeIdentModifier::Prefix(CIBOULETTE_REL_PREFIX)
            }
            Ciboulette2PgAdditionalFieldType::Sorting => {
                Ciboulette2PgSafeIdentModifier::Prefix(CIBOULETTE_SORT_PREFIX)
            }
        }
    }
}

/// An additional field to be included in the selecting CTE
#[derive(Clone, Debug, Getters)]
#[getset(get = "pub")]
pub(crate) struct Ciboulette2PgAdditionalField {
    /// The type of the additional field
    pub(crate) type_: Ciboulette2PgAdditionalFieldType,
    /// The ident to use for the additional field
    pub(crate) ident: Ciboulette2PgTableField,
    /// It's name, later linking
    pub(crate) name: Ciboulette2PgSafeIdent,
    /// The resource type it relates to
    pub(crate) ciboulette_type: Arc<CibouletteResourceType>,
}

impl Ciboulette2PgAdditionalField {
    pub fn new(
        name: Ciboulette2PgSafeIdent,
        ident: Ciboulette2PgTableField,
        type_: Ciboulette2PgAdditionalFieldType,
        ciboulette_type: Arc<CibouletteResourceType>,
    ) -> Self {
        Ciboulette2PgAdditionalField {
            name: name.add_modifier(type_.to_safe_modifier()),
            ident,
            type_,
            ciboulette_type,
        }
    }

    pub fn from_sorting_field(
        el: &CibouletteSortingElement,
        main_type: Arc<CibouletteResourceType>,
    ) -> Result<Self, Ciboulette2PgError> {
        let talbe_field_inner = Ciboulette2PgSafeIdent::try_from(el.field().clone())?;
        let table_field = Ciboulette2PgTableField::new(
            Ciboulette2PgSafeIdentSelector::Single(talbe_field_inner.clone()),
            None,
            None,
        );
        Ok(Ciboulette2PgAdditionalField::new(
            talbe_field_inner,
            table_field,
            Ciboulette2PgAdditionalFieldType::Sorting,
            main_type,
        ))
    }
}
