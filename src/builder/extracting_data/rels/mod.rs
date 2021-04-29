use super::*;

mod many_to_many;
mod many_to_one;

use many_to_many::*;
use many_to_one::*;

pub(crate) fn extract_data_rels<'store, 'request>(
    store: &'store CibouletteStore,
    main_type: Arc<CibouletteResourceType>,
    rel_alias: &str,
    attributes: &'request CibouletteOptionalData<CibouletteResourceIdentifierSelector<'request>>,
) -> Result<Ciboulette2PostgresResourceInformations<'request>, Ciboulette2SqlError>
where
    'store: 'request,
{
    let rel_details = main_type.get_relationship_details(store, rel_alias)?;
    match rel_details.relation_option() {
        CibouletteRelationshipOption::ManyToOne(opt) => {
            extract_many_to_one_relationships_from_ressource_identifiers(
                &attributes,
                opt.clone(),
                rel_details,
            )
        }
        _ => Err(Ciboulette2SqlError::ManyRelationshipDirectWrite),
    }
}

/// Get the relationships data for the main type
pub(crate) fn fill_relationships_without_data(
    acc: &mut Ciboulette2PostgresResourceInformations,
    rel_details: CibouletteResourceRelationshipDetails,
) -> Result<(), Ciboulette2SqlError> {
    match rel_details.relation_option() {
        CibouletteRelationshipOption::ManyToOne(opt) => {
            acc.single_relationships_additional_fields_mut().push(
                Ciboulette2SqlAdditionalField::new(
                    Ciboulette2PostgresTableField::new(
                        Ciboulette2PostgresSafeIdent::try_from(opt.many_resource_key().clone())?,
                        None,
                        None,
                    ),
                    Ciboulette2SqlAdditionalFieldType::Relationship,
                    opt.one_resource().clone(),
                ),
            );
            acc.single_relationships_mut()
                .push(Ciboulette2PostgresResourceSingleRelationships {
                    type_: opt.one_resource().clone(),
                    key: opt.many_resource_key().clone(),
                    rel_details,
                });
        }
        CibouletteRelationshipOption::OneToMany(opt) if opt.part_of_many_to_many().is_none() => {
            acc.multi_relationships_mut().insert(
                rel_details.relation_alias().clone(),
                Ciboulette2PostgresMultiRelationships {
                    type_: rel_details.related_type().clone(),
                    rel_opt: Ciboulette2PostgresMultiRelationshipsType::OneToMany(opt.clone()),
                    values: None,
                    rel_details,
                },
            );
        }
        CibouletteRelationshipOption::ManyToMany(opt) => {
            acc.multi_relationships_mut().insert(
                rel_details.relation_alias().clone(),
                Ciboulette2PostgresMultiRelationships {
                    type_: rel_details.related_type().clone(),
                    rel_opt: Ciboulette2PostgresMultiRelationshipsType::ManyToMany(opt.clone()),
                    values: None,
                    rel_details,
                },
            );
        }
        _ => (),
    };
    Ok(())
}

pub(crate) fn extract_data_from_relationship_details<'request>(
    acc: &mut Ciboulette2PostgresResourceInformations<'request>,
    main_type: &Arc<CibouletteResourceType>,
    relationship_details: CibouletteResourceRelationshipDetails,
    relationship_data: &'request CibouletteRelationshipObject,
    fails_on_many: bool,
) -> Result<(), Ciboulette2SqlError> {
    match relationship_details.relation_option() {
        CibouletteRelationshipOption::ManyToOne(opt) => {
            extract_data_from_relationship_details_many_to_one(
                acc,
                main_type,
                relationship_data,
                opt.clone(),
                relationship_details,
            )?;
        }
        CibouletteRelationshipOption::OneToMany(opt) if !fails_on_many => {
            extract_data_from_relationship_details_many(
                acc,
                relationship_data,
                Ciboulette2PostgresMultiRelationshipsType::OneToMany(opt.clone()),
                relationship_details,
            )
        }
        CibouletteRelationshipOption::ManyToMany(opt) if !fails_on_many => {
            extract_data_from_relationship_details_many(
                acc,
                relationship_data,
                Ciboulette2PostgresMultiRelationshipsType::ManyToMany(opt.clone()),
                relationship_details,
            )
        }
        _ => return Err(Ciboulette2SqlError::ManyRelationshipDirectWrite),
    }
    Ok(())
}
