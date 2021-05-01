use super::*;

/// A table field relating to another resource (i.e. a foreign key)
#[derive(Debug, Clone, Getters)]
#[getset(get = "pub")]
pub(crate) struct Ciboulette2PgRelatingField<'a> {
    /// The field of the current table
    pub(crate) field: Ciboulette2PgTableField,
    /// The table of the relating field
    pub(crate) table: Ciboulette2PgTable,
    /// The relationship chain it refers to
    pub(crate) rel_chain: &'a [CibouletteResourceRelationshipDetails],
    /// The related type it refers to
    pub(crate) related_type: Arc<CibouletteResourceType>,
}

impl<'a> Ciboulette2PgRelatingField<'a> {
    pub fn new(
        field: Ciboulette2PgTableField,
        table: Ciboulette2PgTable,
        rel_chain: &'a [CibouletteResourceRelationshipDetails],
        related_type: Arc<CibouletteResourceType>,
    ) -> Self {
        Ciboulette2PgRelatingField {
            field,
            table,
            rel_chain,
            related_type,
        }
    }
}
