use super::*;

#[derive(Debug, Clone, Getters)]
#[getset(get = "pub")]
pub(crate) struct Ciboulette2PostgresRelatingField<'a> {
    pub(crate) field: Ciboulette2PostgresTableField,
    pub(crate) table: Ciboulette2PostgresTable,
    pub(crate) rel_chain: &'a [CibouletteResourceRelationshipDetails],
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
