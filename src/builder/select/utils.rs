use super::*;

impl<'request> Ciboulette2PostgresBuilder<'request> {
    /// Add additional fields to a final CTE select
    pub(super) fn handle_additionnal_params<'store, 'b, I>(
        &mut self,
        table: &Ciboulette2PostgresTable,
        additional_fields: I,
    ) -> Result<(), Ciboulette2SqlError>
    where
        I: Iterator<Item = &'b Ciboulette2SqlAdditionalField>,
    {
        for field in additional_fields {
            self.buf.write_all(b", ")?;
            self.insert_ident(&field.ident(), table)?;
            self.buf
                .write_all(format!(" AS \"{}\"", field.name()).as_bytes())?;
        }
        Ok(())
    }

    /// Get the relationships data for the main type
    pub(crate) fn get_relationships(
        ciboulette_store: &CibouletteStore,
        main_type: Arc<CibouletteResourceType>,
    ) -> Result<Ciboulette2PostgresResourceInformations<'request>, Ciboulette2SqlError> {
        let mut single_relationships: Vec<Ciboulette2PostgresResourceSingleRelationships> =
            Vec::new();
        let mut multi_relationships: Vec<Ciboulette2PostgresMultiRelationships<'request>> =
            Vec::new();

        for rel_alias in main_type.relationships().keys() {
            let rel = main_type.get_relationship_details(ciboulette_store, rel_alias)?;
            match rel.relation_option() {
                CibouletteRelationshipOption::ManyToOne(opt) => {
                    single_relationships.push(Ciboulette2PostgresResourceSingleRelationships {
                        type_: opt.one_table().clone(),
                        key: opt.many_table_key().clone(),
                    });
                }
                CibouletteRelationshipOption::OneToMany(opt)
                    if opt.part_of_many_to_many().is_none() =>
                {
                    multi_relationships.push(Ciboulette2PostgresMultiRelationships {
                        type_: rel.related_type().clone(),
                        rel_opt: Ciboulette2PostgresMultiRelationshipsType::OneToMany(opt.clone()),
                        values: None,
                    });
                }
                CibouletteRelationshipOption::ManyToMany(opt) => {
                    multi_relationships.push(Ciboulette2PostgresMultiRelationships {
                        type_: rel.related_type().clone(),
                        rel_opt: Ciboulette2PostgresMultiRelationshipsType::ManyToMany(opt.clone()),
                        values: None,
                    });
                }
                _ => continue,
            }
        }
        let mut single_relationships_additional_fields: Vec<Ciboulette2SqlAdditionalField> =
            Vec::with_capacity(single_relationships.len());
        for main_single_rel in single_relationships.iter() {
            single_relationships_additional_fields.push(Ciboulette2SqlAdditionalField::new(
                Ciboulette2PostgresTableField::new(
                    Ciboulette2PostgresSafeIdent::try_from(main_single_rel.key().clone())?,
                    None,
                    None,
                ),
                Ciboulette2SqlAdditionalFieldType::Relationship,
                main_single_rel.type_().clone(),
            ));
        }
        Ok(Ciboulette2PostgresResourceInformations {
            single_relationships,
            single_relationships_additional_fields,
            ..Default::default()
        })
    }
}
