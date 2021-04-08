use super::*;

#[derive(Debug, Clone, Getters)]
#[getset(get = "pub")]
pub(crate) struct Ciboulette2PostgresRelatingField<'store> {
    pub(crate) field: Ciboulette2PostgresTableField<'store>,
    pub(crate) table: Ciboulette2PostgresTable<'store>,
    pub(crate) related_type: Arc<CibouletteResourceType<'store>>,
}

impl<'store> Ciboulette2PostgresRelatingField<'store> {
    pub fn new(
        field: Ciboulette2PostgresTableField<'store>,
        table: Ciboulette2PostgresTable<'store>,
        related_type: Arc<CibouletteResourceType<'store>>,
    ) -> Self {
        Ciboulette2PostgresRelatingField {
            field,
            table,
            related_type,
        }
    }
}
