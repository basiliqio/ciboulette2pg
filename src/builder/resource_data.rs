use super::*;

#[derive(Clone, Debug, Default, Getters)]
#[getset(get = "pub")]
pub(crate) struct Ciboulette2PostgresResourceData<'request> {
    pub provided_values: Vec<(ArcStr, Ciboulette2SqlValue<'request>)>,
    pub single_relationships: Vec<Ciboulette2PostgresMainResourceSingleRels>,
    pub additional_fields: Vec<Ciboulette2SqlAdditionalField>,
}

/// Informations about the main resource type, extracted from the request
#[derive(Clone, Debug, Getters)]
#[getset(get = "pub")]
pub(crate) struct Ciboulette2PostgresMainResourceSingleRels {
    pub type_: Arc<CibouletteResourceType>,
    pub key: ArcStr,
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

fn extract_one_to_many_relationships<'request>(
    from_type: Arc<CibouletteResourceType>,
    to_type: Arc<CibouletteResourceType>,
    rel_obj: &'request CibouletteRelationshipObject<'request>,
    opt: &CibouletteRelationshipOneToManyOption,
) -> Result<Option<(ArcStr, Ciboulette2SqlValue<'request>)>, Ciboulette2SqlError> {
    match rel_obj.data() {
        CibouletteOptionalData::Object(CibouletteResourceIdentifierSelector::One(rel_id)) => {
            Ok(Some((
                opt.many_table_key().clone(),
                Ciboulette2SqlValue::from(rel_id.id()),
            )))
        }
        CibouletteOptionalData::Object(CibouletteResourceIdentifierSelector::Many(_)) => {
            return Err(Ciboulette2SqlError::RequiredSingleRelationship(
                to_type.name().to_string(),
            ));
        }
        CibouletteOptionalData::Null(x) if *x => {
            if !opt.optional() {
                return Err(Ciboulette2SqlError::MissingRelationship(
                    from_type.name().to_string(),
                    to_type.name().to_string(),
                ));
            }
            match opt.one_table().id_type() {
                CibouletteIdType::Number => Ok(Some((
                    opt.many_table_key().clone(),
                    Ciboulette2SqlValue::Numeric(None),
                ))),
                CibouletteIdType::Uuid => Ok(Some((
                    opt.many_table_key().clone(),
                    Ciboulette2SqlValue::Uuid(None),
                ))),
                CibouletteIdType::Text => Ok(Some((
                    opt.many_table_key().clone(),
                    Ciboulette2SqlValue::Text(None),
                ))),
            }
        }
        CibouletteOptionalData::Null(_) => {
            if !opt.optional() {
                return Err(Ciboulette2SqlError::MissingRelationship(
                    from_type.name().to_string(),
                    to_type.name().to_string(),
                ));
            }
            Ok(None)
        }
    }
}

impl<'request> Ciboulette2PostgresResourceData<'request> {
    fn gen_rel_additional_fields(
        main_type: Arc<CibouletteResourceType>,
        single_rels_keys: &[Ciboulette2PostgresMainResourceSingleRels]
    ) -> Result<Vec<Ciboulette2SqlAdditionalField>, Ciboulette2SqlError> {
        let mut single_rels_additional_fields: Vec<Ciboulette2SqlAdditionalField> =
            Vec::with_capacity(single_rels_keys.len());
        for single_rel in single_rels_keys.iter() {
            single_rels_additional_fields.push(Ciboulette2SqlAdditionalField::new(
                Ciboulette2PostgresTableField::new(
                    Ciboulette2PostgresSafeIdent::try_from(single_rel.key().clone())?,
                    None,
                    None,
                ),
                Ciboulette2SqlAdditionalFieldType::Relationship,
                main_type.clone(),
            ))
        }
        Ok(single_rels_additional_fields)
    }
    pub fn parse(
        store: &CibouletteStore,
        main_type: Arc<CibouletteResourceType>,
        attributes: &'request Option<MessyJsonObjectValue<'request>>,
        relationships: &'request BTreeMap<ArcStr, CibouletteRelationshipObject<'request>>,
    ) -> Result<Self, Ciboulette2SqlError> {
        let mut res = Ciboulette2PostgresResourceData::default();

        fill_attributes(&mut res.provided_values, &attributes)?;

        for (rel_alias, rel_obj) in relationships {
            let rel_data = main_type
                .relationships()
                .get(rel_alias)
                .copied()
                .and_then(|x| store.graph().edge_weight(x))
                .ok_or_else(|| {
                    CibouletteError::RelNotInGraph(
                        main_type.name().to_string(),
                        rel_alias.to_string(),
                    )
                })?;
            match rel_data {
                CibouletteRelationshipOption::OneToMany(opt) => {
                    if let Some(value) = extract_one_to_many_relationships(
                        opt.one_table().clone(),
                        opt.many_table().clone(),
                        rel_obj,
                        opt,
                    )? {
                        res.provided_values.push(value);
                    }
                    res.single_relationships
                        .push(Ciboulette2PostgresMainResourceSingleRels {
                            type_: opt.one_table().clone(),
                            key: opt.many_table_key().clone(),
                        });
                }
                CibouletteRelationshipOption::ManyToOne(opt) => {
                    if let Some(value) = extract_one_to_many_relationships(
                        opt.many_table().clone(),
                        opt.one_table().clone(),
                        rel_obj,
                        opt,
                    )? {
                        res.provided_values.push(value);
                    }
                }
                CibouletteRelationshipOption::ManyToMany(opt) => {
                    unimplemented!();
                }
            }
        }
		res.additional_fields = Self::gen_rel_additional_fields(main_type, &res.single_relationships)?;
        Ok(res)
    }
}
