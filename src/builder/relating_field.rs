use super::*;

/// A table field relating to another resource (i.e. a foreign key)
#[derive(Debug, Clone, Getters)]
#[getset(get = "pub")]
pub(crate) struct Ciboulette2PostgresRelatingField<'a> {
    /// The field of the current table
    pub(crate) field: Ciboulette2PostgresTableField,
    /// The table of the relating field
    pub(crate) table: Ciboulette2PostgresTable,
    /// The relationship chain it refers to
    pub(crate) rel_chain: &'a [CibouletteResourceRelationshipDetails],
    /// The related type it refers to
    pub(crate) related_type: Arc<CibouletteResourceType>,
}

impl<'a> Ciboulette2PostgresRelatingField<'a> {
    pub fn new(
        field: Ciboulette2PostgresTableField,
        table: Ciboulette2PostgresTable,
        rel_chain: &'a [CibouletteResourceRelationshipDetails],
        related_type: Arc<CibouletteResourceType>,
    ) -> Self {
        Ciboulette2PostgresRelatingField {
            field,
            table,
            rel_chain,
            related_type,
        }
    }
}
