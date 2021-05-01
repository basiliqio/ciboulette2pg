use super::*;

mod many_to_many;
mod many_to_one;

use many_to_many::*;
use many_to_one::*;

/// Extract the many-to-one data from the request
/// Fails if the relation is not many-to-one
pub(crate) fn extract_data_rels<'store, 'request>(
    store: &'store CibouletteStore,
    main_type: Arc<CibouletteResourceType>,
    rel_alias: &str,
    attributes: &'request CibouletteOptionalData<CibouletteResourceIdentifierSelector<'request>>,
) -> Result<Ciboulette2PgResourceInformations<'request>, Ciboulette2PgError>
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
        _ => Err(Ciboulette2PgError::ManyRelationshipDirectWrite),
    }
}

/// Get the relationships data for the main type, don't extract data from that
/// relationships
pub(crate) fn fill_relationships_without_data(
    acc: &mut Ciboulette2PgResourceInformations,
    rel_details: CibouletteResourceRelationshipDetails,
) -> Result<(), Ciboulette2PgError> {
    match rel_details.relation_option() {
        CibouletteRelationshipOption::ManyToOne(opt) => {
            acc.single_relationships_additional_fields_mut().push(
                Ciboulette2PgAdditionalField::new(
                    Ciboulette2PgTableField::new(
                        Ciboulette2PgSafeIdent::try_from(opt.many_resource_key().clone())?,
                        None,
                        None,
                    ),
                    Ciboulette2PgAdditionalFieldType::Relationship,
                    opt.one_resource().clone(),
                ),
            );
            acc.single_relationships_mut()
                .push(Ciboulette2PgResourceSingleRelationships {
                    type_: opt.one_resource().clone(),
                    key: opt.many_resource_key().clone(),
                    rel_details,
                });
        }
        CibouletteRelationshipOption::OneToMany(opt) if opt.part_of_many_to_many().is_none() => {
            acc.multi_relationships_mut().insert(
                rel_details.relation_alias().clone(),
                Ciboulette2PgMultiRelationships {
                    type_: rel_details.related_type().clone(),
                    rel_opt: Ciboulette2PgMultiRelationshipsType::OneToMany(opt.clone()),
                    values: None,
                    rel_details,
                },
            );
        }
        CibouletteRelationshipOption::ManyToMany(opt) => {
            acc.multi_relationships_mut().insert(
                rel_details.relation_alias().clone(),
                Ciboulette2PgMultiRelationships {
                    type_: rel_details.related_type().clone(),
                    rel_opt: Ciboulette2PgMultiRelationshipsType::ManyToMany(opt.clone()),
                    values: None,
                    rel_details,
                },
            );
        }
        _ => (),
    };
    Ok(())
}

/// Extract relationships data from the request for all relationships types
pub(crate) fn extract_data_from_relationship_details<'request>(
    acc: &mut Ciboulette2PgResourceInformations<'request>,
    main_type: &Arc<CibouletteResourceType>,
    relationship_details: CibouletteResourceRelationshipDetails,
    relationship_data: &'request CibouletteRelationshipObject,
    fails_on_many: bool,
) -> Result<(), Ciboulette2PgError> {
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
                Ciboulette2PgMultiRelationshipsType::OneToMany(opt.clone()),
                relationship_details,
            )
        }
        CibouletteRelationshipOption::ManyToMany(opt) if !fails_on_many => {
            extract_data_from_relationship_details_many(
                acc,
                relationship_data,
                Ciboulette2PgMultiRelationshipsType::ManyToMany(opt.clone()),
                relationship_details,
            )
        }
        _ => return Err(Ciboulette2PgError::ManyRelationshipDirectWrite),
    }
    Ok(())
}
