use super::*;
pub mod main;
pub mod rel;

impl<'store, 'request> Ciboulette2PostgresBuilder<'request> {
    /// Generate a SQL query to handle a `DELETE` request
    ///
    /// Fails if trying to delete an one-to-many relationships.
    /// Fails if trying to delete a non optional many-to-one relationships.
    pub fn gen_delete(
        store: &'store CibouletteStore,
        table_store: &'store Ciboulette2PostgresTableStore,
        request: &'request CibouletteDeleteRequest<'request>,
    ) -> Result<Self, Ciboulette2SqlError>
    where
        'store: 'request,
    {
        let mut se = Ciboulette2PostgresBuilder::default();
        match request.related_type() {
            Some(related_type) => {
                se.handle_delete_rel(request, related_type, store, table_store)?
            }
            None => se.gen_delete_normal(&table_store, request), // If we're not deleting a relationships but an object
        }?;
        Ok(se)
    }

    /// Handle deleting Many-to-One relationships, extracting the right parameters from the query.
    fn handle_delete_rel(
        &mut self,
        request: &'request CibouletteDeleteRequest,
        related_type: &Arc<CibouletteResourceType>,
        store: &CibouletteStore,
        table_store: &Ciboulette2PostgresTableStore,
    ) -> Result<Result<(), Ciboulette2SqlError>, Ciboulette2SqlError> {
        let alias = request
            .resource_type()
            .get_alias(related_type.name().as_str())?;
        let (_, opt) = store.get_rel(request.resource_type().name().as_str(), alias.as_str())?;
        Ok(match opt {
            CibouletteRelationshipOption::ManyToOne(opt)
            | CibouletteRelationshipOption::OneToMany(opt)
                if opt.many_resource().as_ref() == request.resource_type().as_ref() // If the deleted type is the many part of the one-to-many
					&& opt.one_resource().as_ref() == related_type.as_ref() // If the deleted related type is the one part of the one-to-many
					&& *opt.optional() =>
            // If the field is optional
            {
                self.gen_delete_rel_one_to_many(&table_store, request, opt)
            }
            CibouletteRelationshipOption::ManyToOne(opt)
            | CibouletteRelationshipOption::OneToMany(opt)
                if opt.many_resource().as_ref() == request.resource_type().as_ref()
                    && opt.one_resource().as_ref() == related_type.as_ref() =>
            {
                return Err(Ciboulette2SqlError::MissingRelationship(
                    request.resource_type().name().to_string(),
                    opt.one_resource().name().to_string(),
                ));
            }
            _ => return Err(Ciboulette2SqlError::ManyRelationshipDirectWrite), // Fails if it's a multi relationship
        })
    }
}
