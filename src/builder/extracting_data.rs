use super::*;

/// Informations about the main resource type, extracted from the request
#[derive(Clone, Debug, Getters)]
#[getset(get = "pub")]
pub(crate) struct Ciboulette2PostgresResourceSingleRelationships {
    pub type_: Arc<CibouletteResourceType>,
    pub key: ArcStr,
}

#[derive(Clone, Debug, Getters)]
#[getset(get = "pub")]
pub(crate) struct Ciboulette2PostgresMultiRelationships<'request> {
    pub type_: Arc<CibouletteResourceType>,
    pub rel_opt: Ciboulette2PostgresMultiRelationshipsType,
    pub values: Option<Vec<Ciboulette2SqlValue<'request>>>,
}

/// Extract informations concerning the main resource's one-to-many relationships
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Ciboulette2PostgresMultiRelationshipsType {
    OneToMany(Arc<CibouletteRelationshipOneToManyOption>),
    ManyToMany(Arc<CibouletteRelationshipManyToManyOption>),
}

impl Ciboulette2PostgresMultiRelationshipsType {
    pub fn dest_resource(&self) -> &CibouletteResourceType {
        match self {
            Ciboulette2PostgresMultiRelationshipsType::OneToMany(x) => x.one_table(),
            Ciboulette2PostgresMultiRelationshipsType::ManyToMany(x) => x.bucket_resource(),
        }
    }

    pub fn dest_key(
        &self,
        main_type: &CibouletteResourceType,
    ) -> Result<ArcStr, CibouletteError> {
        match self {
            Ciboulette2PostgresMultiRelationshipsType::OneToMany(x) => {
                Ok(x.many_table_key().clone())
            }
            Ciboulette2PostgresMultiRelationshipsType::ManyToMany(x) => x.keys_for_type(main_type),
        }
    }
}

/// Informations about the main resource type, extracted from the request
#[derive(Clone, Debug, Default, Getters, MutGetters)]
#[getset(get = "pub", get_mut = "pub")]
pub(crate) struct Ciboulette2PostgresResourceInformations<'request> {
    pub values: Vec<(ArcStr, Ciboulette2SqlValue<'request>)>,
    pub single_relationships: Vec<Ciboulette2PostgresResourceSingleRelationships>,
    pub single_relationships_additional_fields: Vec<Ciboulette2SqlAdditionalField>,
    pub multi_relationships: BTreeMap<ArcStr, Ciboulette2PostgresMultiRelationships<'request>>,
}

/// Extract attributes from the request and push them to an arguments vector
/// compatible with SQLx for later execution
pub fn fill_attributes<'store, 'request>(
    args: &mut Vec<(ArcStr, Ciboulette2SqlValue<'request>)>,
    obj: &'request Option<MessyJsonObjectValue<'store>>,
) -> Result<(), Ciboulette2SqlError> {
    if let Some(obj) = obj {
        for (k, v) in obj.iter() {
            if matches!(v, MessyJsonValue::Null(MessyJsonNullType::Absent, _)) {
                continue;
            }
            // Iterate over every attribute
            args.push((k.clone(), Ciboulette2SqlValue::try_from(v)?));
        }
    }
    Ok(())
}

pub(crate) fn extract_data_from_relationship_details<'store, 'request>(
    acc: &mut Ciboulette2PostgresResourceInformations<'request>,
    store: &'store CibouletteStore,
    main_type: &Arc<CibouletteResourceType>,
    relationship_details: &CibouletteResourceRelationshipDetails,
    relationship_data: &'request CibouletteRelationshipObject,
    fails_on_many: bool,
) -> Result<(), Ciboulette2SqlError> {
    match relationship_details.relation_option() {
        CibouletteRelationshipOption::ManyToOne(opt) => {
            extract_data_from_relationship_details_many_to_one(
                acc,
                main_type,
                relationship_data,
                opt,
                relationship_details,
            )?;
        }
        CibouletteRelationshipOption::OneToMany(opt) if !fails_on_many => {
            extract_data_from_relationship_details_many(
                acc,
                main_type,
                relationship_data,
                Ciboulette2PostgresMultiRelationshipsType::OneToMany(opt.clone()),
                relationship_details,
            )
        }
        CibouletteRelationshipOption::ManyToMany(opt) if !fails_on_many => {
            extract_data_from_relationship_details_many(
                acc,
                main_type,
                relationship_data,
                Ciboulette2PostgresMultiRelationshipsType::ManyToMany(opt.clone()),
                relationship_details,
            )
        }
        _ => return Err(Ciboulette2SqlError::ManyRelationshipDirectWrite),
    }
    Ok(())
}

fn extract_data_from_relationship_details_many_to_one<'request>(
    acc: &mut Ciboulette2PostgresResourceInformations<'request>,
    main_type: &Arc<CibouletteResourceType>,
    relationship_data: &'request CibouletteRelationshipObject,
    opt: &CibouletteRelationshipOneToManyOption,
    relationship_details: &CibouletteResourceRelationshipDetails,
) -> Result<(), Ciboulette2SqlError> {
    match relationship_data.data() {
        CibouletteOptionalData::Object(CibouletteResourceIdentifierSelector::One(rel_id)) => {
            acc.values_mut().push((
                opt.many_table_key().clone(),
                Ciboulette2SqlValue::from(rel_id.id()),
            ));
        }
        CibouletteOptionalData::Object(CibouletteResourceIdentifierSelector::Many(_)) => {
            return Err(Ciboulette2SqlError::RequiredSingleRelationship(
                relationship_details.relation_alias().to_string(),
            ));
        }
        CibouletteOptionalData::Null(x) if *x => {
            if !opt.optional() {
                return Err(Ciboulette2SqlError::MissingRelationship(
                    main_type.name().to_string(),
                    relationship_details.relation_alias().to_string(),
                ));
            }
            match opt.one_table().id_type() {
                CibouletteIdType::Number => acc.values_mut().push((
                    opt.many_table_key().clone(),
                    Ciboulette2SqlValue::Numeric(None),
                )),
                CibouletteIdType::Uuid => acc.values_mut().push((
                    opt.many_table_key().clone(),
                    Ciboulette2SqlValue::Uuid(None),
                )),
                CibouletteIdType::Text => acc.values_mut().push((
                    opt.many_table_key().clone(),
                    Ciboulette2SqlValue::Text(None),
                )),
            }
        }
        CibouletteOptionalData::Null(_) => {
            if !opt.optional() {
                return Err(Ciboulette2SqlError::MissingRelationship(
                    main_type.name().to_string(),
                    relationship_details.relation_alias().to_string(),
                ));
            }
        }
    }
    acc.single_relationships_mut()
        .push(Ciboulette2PostgresResourceSingleRelationships {
            type_: opt.one_table().clone(),
            key: opt.many_table_key().clone(),
        });
    acc.single_relationships_additional_fields_mut()
        .push(Ciboulette2SqlAdditionalField::new(
            Ciboulette2PostgresTableField::new(
                Ciboulette2PostgresSafeIdent::try_from(opt.many_table_key().clone())?,
                None,
                None,
            ),
            Ciboulette2SqlAdditionalFieldType::Relationship,
            opt.one_table().clone(),
        ));
    Ok(())
}

fn extract_data_from_relationship_details_many<'request>(
    mut acc: &mut Ciboulette2PostgresResourceInformations<'request>,
    main_type: &Arc<CibouletteResourceType>,
    relationship_data: &'request CibouletteRelationshipObject,
    rel_opt: Ciboulette2PostgresMultiRelationshipsType,
    relationship_details: &CibouletteResourceRelationshipDetails,
) {
    match relationship_data.data() {
        CibouletteOptionalData::Object(CibouletteResourceIdentifierSelector::One(rel_id)) => {
            acc.multi_relationships_mut().insert(
                relationship_details.relation_alias().clone(),
                Ciboulette2PostgresMultiRelationships {
                    type_: relationship_details.related_type().clone(),
                    rel_opt,
                    values: Some(vec![Ciboulette2SqlValue::from(rel_id.id())]),
                },
            );
        }
        CibouletteOptionalData::Object(CibouletteResourceIdentifierSelector::Many(rels_id)) => {
            acc.multi_relationships_mut().insert(
                relationship_details.relation_alias().clone(),
                Ciboulette2PostgresMultiRelationships {
                    type_: relationship_details.related_type().clone(),
                    rel_opt,
                    values: Some(
                        rels_id
                            .iter()
                            .map(|x| Ciboulette2SqlValue::from(x.id()))
                            .collect(),
                    ),
                },
            );
        }
        CibouletteOptionalData::Null(x) if *x => {
            acc.multi_relationships_mut().insert(
                relationship_details.relation_alias().clone(),
                Ciboulette2PostgresMultiRelationships {
                    type_: relationship_details.related_type().clone(),
                    rel_opt,
                    values: Some(vec![Ciboulette2SqlValue::Text(None)]), // FIXME Not always TEXT
                },
            );
        }
        CibouletteOptionalData::Null(_) => {
            acc.multi_relationships_mut().insert(
                relationship_details.relation_alias().clone(),
                Ciboulette2PostgresMultiRelationships {
                    type_: relationship_details.related_type().clone(),
                    rel_opt,
                    values: None,
                },
            );
        }
    }
}

/// Get the relationships data for the main type
pub(crate) fn fill_relationships_without_data<'request>(
    acc: &mut Ciboulette2PostgresResourceInformations<'request>,
    rel_detail: &CibouletteResourceRelationshipDetails,
) -> Result<(), Ciboulette2SqlError> {
    match rel_detail.relation_option() {
        CibouletteRelationshipOption::ManyToOne(opt) => {
            acc.single_relationships_mut()
                .push(Ciboulette2PostgresResourceSingleRelationships {
                    type_: opt.one_table().clone(),
                    key: opt.many_table_key().clone(),
                });
            acc.single_relationships_additional_fields_mut().push(
                Ciboulette2SqlAdditionalField::new(
                    Ciboulette2PostgresTableField::new(
                        Ciboulette2PostgresSafeIdent::try_from(opt.many_table_key().clone())?,
                        None,
                        None,
                    ),
                    Ciboulette2SqlAdditionalFieldType::Relationship,
                    opt.one_table().clone(),
                ),
            );
        }
        CibouletteRelationshipOption::OneToMany(opt) if opt.part_of_many_to_many().is_none() => {
            acc.multi_relationships_mut().insert(
                rel_detail.relation_alias().clone(),
                Ciboulette2PostgresMultiRelationships {
                    type_: rel_detail.related_type().clone(),
                    rel_opt: Ciboulette2PostgresMultiRelationshipsType::OneToMany(opt.clone()),
                    values: None,
                },
            );
        }
        CibouletteRelationshipOption::ManyToMany(opt) => {
            acc.multi_relationships_mut().insert(
                rel_detail.relation_alias().clone(),
                Ciboulette2PostgresMultiRelationships {
                    type_: rel_detail.related_type().clone(),
                    rel_opt: Ciboulette2PostgresMultiRelationshipsType::ManyToMany(opt.clone()),
                    values: None,
                },
            );
        }
        _ => (),
    };
    Ok(())
}

pub(crate) fn extract_data<'store, 'request>(
    store: &'store CibouletteStore,
    main_type: Arc<CibouletteResourceType>,
    attributes: &'request Option<MessyJsonObjectValue<'request>>,
    relationships: &'request BTreeMap<ArcStr, CibouletteRelationshipObject<'request>>,
    fails_on_many: bool,
) -> Result<Ciboulette2PostgresResourceInformations<'request>, Ciboulette2SqlError>
where
    'store: 'request,
{
    let mut res = Ciboulette2PostgresResourceInformations::default();
    fill_attributes(&mut res.values, &attributes)?;

    for rel_alias in main_type.relationships().keys() {
        let rel_details = main_type.get_relationship_details(store, rel_alias)?;
        match relationships.get(rel_alias) {
            Some(rel_data) => extract_data_from_relationship_details(
                &mut res,
                store,
                &main_type,
                &rel_details,
                rel_data,
                fails_on_many,
            )?,
            None => fill_relationships_without_data(&mut res, &rel_details)?,
        }
    }
    Ok(res)
}

pub(crate) fn extract_data_no_body<'store, 'request>(
    store: &'store CibouletteStore,
    main_type: Arc<CibouletteResourceType>,
) -> Result<Ciboulette2PostgresResourceInformations<'request>, Ciboulette2SqlError>
where
    'store: 'request,
{
    let mut res = Ciboulette2PostgresResourceInformations::default();
    for rel_alias in main_type.relationships().keys() {
        let rel_details = main_type.get_relationship_details(store, rel_alias)?;
        fill_relationships_without_data(&mut res, &rel_details)?
    }
    Ok(res)
}

/// Extract data from a single relationship object
fn extract_many_to_one_relationships_from_ressource_identifiers<'request>(
    attributes: &'request CibouletteOptionalData<CibouletteResourceIdentifierSelector<'request>>,
    rel_opt: &Arc<CibouletteRelationshipOneToManyOption>,
) -> Result<Ciboulette2PostgresResourceInformations<'request>, Ciboulette2SqlError> {
    match attributes {
        CibouletteOptionalData::Object(CibouletteResourceIdentifierSelector::One(rel_id)) => {
            Ok(Ciboulette2PostgresResourceInformations {
                values: vec![(
                    rel_opt.many_table_key().clone(),
                    Ciboulette2SqlValue::from(rel_id.id()),
                )],
                single_relationships: vec![Ciboulette2PostgresResourceSingleRelationships {
                    type_: rel_opt.one_table().clone(),
                    key: rel_opt.many_table_key().clone(),
                }],
                single_relationships_additional_fields: vec![Ciboulette2SqlAdditionalField::new(
                    Ciboulette2PostgresTableField::new(
                        Ciboulette2PostgresSafeIdent::try_from(rel_opt.many_table_key().clone())?,
                        None,
                        None,
                    ),
                    Ciboulette2SqlAdditionalFieldType::Relationship,
                    rel_opt.one_table().clone(),
                )],
                multi_relationships: BTreeMap::default(),
            })
        }
        CibouletteOptionalData::Object(_) => {
            Err(Ciboulette2SqlError::MultiIdsForSingleRelationships)
        }
        CibouletteOptionalData::Null(x) if *x => Ok(Ciboulette2PostgresResourceInformations {
            values: vec![(
                rel_opt.many_table_key().clone(),
                match rel_opt.one_table().id_type() {
                    CibouletteIdType::Text => Ciboulette2SqlValue::Text(None),
                    CibouletteIdType::Number => Ciboulette2SqlValue::Numeric(None),
                    CibouletteIdType::Uuid => Ciboulette2SqlValue::Uuid(None),
                },
            )],
            single_relationships: vec![Ciboulette2PostgresResourceSingleRelationships {
                type_: rel_opt.one_table().clone(),
                key: rel_opt.many_table_key().clone(),
            }],
            single_relationships_additional_fields: vec![Ciboulette2SqlAdditionalField::new(
                Ciboulette2PostgresTableField::new(
                    Ciboulette2PostgresSafeIdent::try_from(rel_opt.many_table_key().clone())?,
                    None,
                    None,
                ),
                Ciboulette2SqlAdditionalFieldType::Relationship,
                rel_opt.one_table().clone(),
            )],
            multi_relationships: BTreeMap::default(),
        }),
        CibouletteOptionalData::Null(_) => Ok(Ciboulette2PostgresResourceInformations::default()),
    }
}

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
            extract_many_to_one_relationships_from_ressource_identifiers(&attributes, &opt)
        }
        _ => Err(Ciboulette2SqlError::ManyRelationshipDirectWrite),
    }
}
