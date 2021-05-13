use super::*;

/// Extract data from a single relationship object
pub(super) fn extract_many_to_one_relationships_from_ressource_identifiers<'request>(
    attributes: &'request CibouletteOptionalData<CibouletteResourceIdentifierSelector<'request>>,
    rel_opt: Arc<CibouletteRelationshipOneToManyOption>,
    rel_details: CibouletteResourceRelationshipDetails,
) -> Result<Ciboulette2PgResourceInformations<'request>, Ciboulette2PgError> {
    match attributes {
        CibouletteOptionalData::Object(CibouletteResourceIdentifierSelector::One(rel_id)) => {
            let many_resource_key_safe_ident =
                Ciboulette2PgSafeIdent::try_from(rel_opt.many_resource_key().clone())?;

            Ok(Ciboulette2PgResourceInformations {
                values: vec![(
                    rel_opt.many_resource_key().clone(),
                    Ciboulette2PgValue::from(rel_id.id().get(0)?), // There can only be one id
                )],
                single_relationships: vec![Ciboulette2PgResourceSingleRelationships {
                    type_: rel_opt.one_resource().clone(),
                    key: rel_opt.many_resource_key().clone(),
                    rel_details,
                }],
                single_relationships_additional_fields: vec![Ciboulette2PgAdditionalField::new(
                    many_resource_key_safe_ident.clone(),
                    Ciboulette2PgTableField::new(
                        Ciboulette2PgSafeIdentSelector::Single(many_resource_key_safe_ident),
                        None,
                        None,
                    ),
                    Ciboulette2PgAdditionalFieldType::Relationship,
                    rel_opt.one_resource().clone(),
                )],
                multi_relationships: BTreeMap::default(),
            })
        }
        CibouletteOptionalData::Object(_) => {
            Err(Ciboulette2PgError::MultiIdsForSingleRelationships)
        }
        CibouletteOptionalData::Null(x) if *x => {
            let one_resource_key_safe_ident =
                Ciboulette2PgSafeIdent::try_from(rel_opt.many_resource_key().clone())?;

            Ok(Ciboulette2PgResourceInformations {
                values: vec![(
                    rel_opt.many_resource_key().clone(),
                    // There can only be one id
                    match rel_opt.one_resource().ids().get(0)? {
                        CibouletteIdType::Text(_) => Ciboulette2PgValue::Text(None),
                        CibouletteIdType::Number(_) => Ciboulette2PgValue::Numeric(None),
                        CibouletteIdType::Uuid(_) => Ciboulette2PgValue::Uuid(None),
                    },
                )],
                single_relationships: vec![Ciboulette2PgResourceSingleRelationships {
                    type_: rel_opt.one_resource().clone(),
                    key: rel_opt.many_resource_key().clone(),
                    rel_details,
                }],
                single_relationships_additional_fields: vec![Ciboulette2PgAdditionalField::new(
                    one_resource_key_safe_ident.clone(),
                    Ciboulette2PgTableField::new(
                        Ciboulette2PgSafeIdentSelector::Single(one_resource_key_safe_ident),
                        None,
                        None,
                    ),
                    Ciboulette2PgAdditionalFieldType::Relationship,
                    rel_opt.one_resource().clone(),
                )],
                multi_relationships: BTreeMap::default(),
            })
        }
        CibouletteOptionalData::Null(_) => Ok(Ciboulette2PgResourceInformations::default()),
    }
}

/// Extract many-to-one relationships id from requests
pub(super) fn extract_data_from_relationship_details_many_to_one<'request>(
    acc: &mut Ciboulette2PgResourceInformations<'request>,
    main_type: &Arc<CibouletteResourceType>,
    relationship_data: &'request CibouletteRelationshipObject,
    opt: Arc<CibouletteRelationshipOneToManyOption>,
    rel_details: CibouletteResourceRelationshipDetails,
) -> Result<(), Ciboulette2PgError> {
    match relationship_data.data() {
        CibouletteOptionalData::Object(CibouletteResourceIdentifierSelector::One(rel_id)) => {
            acc.values_mut().push((
                opt.many_resource_key().clone(),
                Ciboulette2PgValue::from(rel_id.id().get(0)?),
            ));
        }
        CibouletteOptionalData::Object(CibouletteResourceIdentifierSelector::Many(_)) => {
            return Err(Ciboulette2PgError::RequiredSingleRelationship(
                rel_details.relation_alias().to_string(),
            ));
        }
        CibouletteOptionalData::Null(x) if *x => {
            if !opt.optional() {
                return Err(Ciboulette2PgError::MissingRelationship(
                    main_type.name().to_string(),
                    rel_details.relation_alias().to_string(),
                ));
            }
            match opt.one_resource().ids().get(0)? {
                CibouletteIdType::Number(_) => acc.values_mut().push((
                    opt.many_resource_key().clone(),
                    Ciboulette2PgValue::Numeric(None),
                )),
                CibouletteIdType::Uuid(_) => acc.values_mut().push((
                    opt.many_resource_key().clone(),
                    Ciboulette2PgValue::Uuid(None),
                )),
                CibouletteIdType::Text(_) => acc.values_mut().push((
                    opt.many_resource_key().clone(),
                    Ciboulette2PgValue::Text(None),
                )),
            }
        }
        CibouletteOptionalData::Null(_) => {
            if !opt.optional() {
                return Err(Ciboulette2PgError::MissingRelationship(
                    main_type.name().to_string(),
                    rel_details.relation_alias().to_string(),
                ));
            }
        }
    }
    let many_resource_key_safe_ident =
        Ciboulette2PgSafeIdent::try_from(opt.many_resource_key().clone())?;

    acc.single_relationships_mut()
        .push(Ciboulette2PgResourceSingleRelationships {
            type_: opt.one_resource().clone(),
            key: opt.many_resource_key().clone(),
            rel_details,
        });
    acc.single_relationships_additional_fields_mut()
        .push(Ciboulette2PgAdditionalField::new(
            many_resource_key_safe_ident.clone(),
            Ciboulette2PgTableField::new(
                Ciboulette2PgSafeIdentSelector::Single(many_resource_key_safe_ident),
                None,
                None,
            ),
            Ciboulette2PgAdditionalFieldType::Relationship,
            opt.one_resource().clone(),
        ));
    Ok(())
}
