use super::*;
pub mod main;
pub mod rel;

impl<'a> Ciboulette2PostgresBuilder<'a> {
    /// Generate a SQL query to handle a `DELETE` request
    ///
    /// Fails if trying to delete an one-to-many relationships.
    /// Fails if trying to delete a non optional one-to-one relationships.
    pub fn gen_delete(
        store: &'a CibouletteStore<'a>,
        table_store: &'a Ciboulette2PostgresTableStore<'a>,
        query: &'a CibouletteDeleteRequest<'a>,
    ) -> Result<Self, Ciboulette2SqlError> {
        let mut se = Ciboulette2PostgresBuilder::default();
        match query.related_type() {
            Some(related_type) => {
                let alias = query
                    .resource_type()
                    .get_alias(related_type.name().as_str())?; // Get the alias
                let (_, opt) =
                    store.get_rel(query.resource_type().name().as_str(), alias.as_str())?; // Get the relationship
                match opt {
                    CibouletteRelationshipOption::One(opt) if *opt.optional() => {
                        // If it's an single optional value, go ahed
                        se.gen_delete_rel(&table_store, query, opt)
                    }
                    CibouletteRelationshipOption::One(opt) => {
                        // Fails if it's not optional
                        return Err(Ciboulette2SqlError::RequiredRelationship(
                            query.resource_type().name().clone(),
                            opt.key().clone(),
                        ));
                    }
                    _ => return Err(Ciboulette2SqlError::BulkRelationshipDelete), // Fails if it's a multi relationship
                }
            }
            None => se.gen_delete_normal(&table_store, query), // If we're not deleting a relationships but an object
        }?;
        Ok(se)
    }
}
