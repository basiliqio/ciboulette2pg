use super::*;

#[derive(Debug, Clone, Getters)]
#[getset(get = "pub")]
pub(crate) struct Ciboulette2PostgresRelatingField<'a> {
    pub(crate) field: Ciboulette2PostgresTableField<'a>,
    pub(crate) table: Ciboulette2PostgresTable<'a>,
    pub(crate) related_type: Arc<CibouletteResourceType<'a>>,
}

impl<'a> Ciboulette2PostgresRelatingField<'a> {
    pub fn new(
        field: Ciboulette2PostgresTableField<'a>,
        table: Ciboulette2PostgresTable<'a>,
        related_type: Arc<CibouletteResourceType<'a>>,
    ) -> Self {
        Ciboulette2PostgresRelatingField {
            field,
            table,
            related_type,
        }
    }
}
