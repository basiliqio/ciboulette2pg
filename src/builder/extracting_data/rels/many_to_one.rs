use super::*;

/// Extract data from a single relationship object
pub(super) fn extract_many_to_one_relationships_from_ressource_identifiers<'request>(
    attributes: &'request CibouletteOptionalData<CibouletteResourceIdentifierSelector<'request>>,
    rel_opt: Arc<CibouletteRelationshipOneToManyOption>,
    rel_details: CibouletteResourceRelationshipDetails,
) -> Result<Ciboulette2PostgresResourceInformations<'request>, Ciboulette2SqlError> {
    match attributes {
        CibouletteOptionalData::Object(CibouletteResourceIdentifierSelector::One(rel_id)) => {
            Ok(Ciboulette2PostgresResourceInformations {
                values: vec![(
                    rel_opt.many_resource_key().clone(),
                    Ciboulette2SqlValue::from(rel_id.id()),
                )],
                single_relationships: vec![Ciboulette2PostgresResourceSingleRelationships {
                    type_: rel_opt.one_resource().clone(),
                    key: rel_opt.many_resource_key().clone(),
                    rel_details,
                }],
                single_relationships_additional_fields: vec![Ciboulette2SqlAdditionalField::new(
                    Ciboulette2PostgresTableField::new(
                        Ciboulette2PostgresSafeIdent::try_from(
                            rel_opt.many_resource_key().clone(),
                        )?,
                        None,
                        None,
                    ),
                    Ciboulette2SqlAdditionalFieldType::Relationship,
                    rel_opt.one_resource().clone(),
                )],
                multi_relationships: BTreeMap::default(),
            })
        }
        CibouletteOptionalData::Object(_) => {
            Err(Ciboulette2SqlError::MultiIdsForSingleRelationships)
        }
        CibouletteOptionalData::Null(x) if *x => Ok(Ciboulette2PostgresResourceInformations {
            values: vec![(
                rel_opt.many_resource_key().clone(),
                match rel_opt.one_resource().id_type() {
                    CibouletteIdType::Text => Ciboulette2SqlValue::Text(None),
                    CibouletteIdType::Number => Ciboulette2SqlValue::Numeric(None),
                    CibouletteIdType::Uuid => Ciboulette2SqlValue::Uuid(None),
                },
            )],
            single_relationships: vec![Ciboulette2PostgresResourceSingleRelationships {
                type_: rel_opt.one_resource().clone(),
                key: rel_opt.many_resource_key().clone(),
                rel_details,
            }],
            single_relationships_additional_fields: vec![Ciboulette2SqlAdditionalField::new(
                Ciboulette2PostgresTableField::new(
                    Ciboulette2PostgresSafeIdent::try_from(rel_opt.many_resource_key().clone())?,
                    None,
                    None,
                ),
                Ciboulette2SqlAdditionalFieldType::Relationship,
                rel_opt.one_resource().clone(),
            )],
            multi_relationships: BTreeMap::default(),
        }),
        CibouletteOptionalData::Null(_) => Ok(Ciboulette2PostgresResourceInformations::default()),
    }
}

/// Extract many-to-one relationships id from requests
pub(super) fn extract_data_from_relationship_details_many_to_one<'request>(
    acc: &mut Ciboulette2PostgresResourceInformations<'request>,
    main_type: &Arc<CibouletteResourceType>,
    relationship_data: &'request CibouletteRelationshipObject,
    opt: Arc<CibouletteRelationshipOneToManyOption>,
    rel_details: CibouletteResourceRelationshipDetails,
) -> Result<(), Ciboulette2SqlError> {
    match relationship_data.data() {
        CibouletteOptionalData::Object(CibouletteResourceIdentifierSelector::One(rel_id)) => {
            acc.values_mut().push((
                opt.many_resource_key().clone(),
                Ciboulette2SqlValue::from(rel_id.id()),
            ));
        }
        CibouletteOptionalData::Object(CibouletteResourceIdentifierSelector::Many(_)) => {
            return Err(Ciboulette2SqlError::RequiredSingleRelationship(
                rel_details.relation_alias().to_string(),
            ));
        }
        CibouletteOptionalData::Null(x) if *x => {
            if !opt.optional() {
                return Err(Ciboulette2SqlError::MissingRelationship(
                    main_type.name().to_string(),
                    rel_details.relation_alias().to_string(),
                ));
            }
            match opt.one_resource().id_type() {
                CibouletteIdType::Number => acc.values_mut().push((
                    opt.many_resource_key().clone(),
                    Ciboulette2SqlValue::Numeric(None),
                )),
                CibouletteIdType::Uuid => acc.values_mut().push((
                    opt.many_resource_key().clone(),
                    Ciboulette2SqlValue::Uuid(None),
                )),
                CibouletteIdType::Text => acc.values_mut().push((
                    opt.many_resource_key().clone(),
                    Ciboulette2SqlValue::Text(None),
                )),
            }
        }
        CibouletteOptionalData::Null(_) => {
            if !opt.optional() {
                return Err(Ciboulette2SqlError::MissingRelationship(
                    main_type.name().to_string(),
                    rel_details.relation_alias().to_string(),
                ));
            }
        }
    }
    acc.single_relationships_mut()
        .push(Ciboulette2PostgresResourceSingleRelationships {
            type_: opt.one_resource().clone(),
            key: opt.many_resource_key().clone(),
            rel_details,
        });
    acc.single_relationships_additional_fields_mut()
        .push(Ciboulette2SqlAdditionalField::new(
            Ciboulette2PostgresTableField::new(
                Ciboulette2PostgresSafeIdent::try_from(opt.many_resource_key().clone())?,
                None,
                None,
            ),
            Ciboulette2SqlAdditionalFieldType::Relationship,
            opt.one_resource().clone(),
        ));
    Ok(())
}
