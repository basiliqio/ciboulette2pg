use super::*;

#[derive(Debug, Clone, Getters)]
#[getset(get = "pub")]
pub(crate) struct Ciboulette2PostgresRelatingField {
    pub(crate) field: Ciboulette2PostgresTableField,
    pub(crate) table: Ciboulette2PostgresTable,
    pub(crate) related_type: Arc<CibouletteResourceType>,
}

impl Ciboulette2PostgresRelatingField {
    pub fn new(
        field: Ciboulette2PostgresTableField,
        table: Ciboulette2PostgresTable,
        related_type: Arc<CibouletteResourceType>,
    ) -> Self {
        Ciboulette2PostgresRelatingField {
            field,
            table,
            related_type,
        }
    }
}
