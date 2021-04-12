use super::*;
use main::Ciboulette2PostgresMainResourceInformations;
/// Extract informations concerning the main resource's one-to-many relationships
#[derive(Clone, Debug, Getters)]
#[getset(get = "pub")]
pub(crate) struct Ciboulette2PostgresMainResourceRelationships<'request> {
    pub type_: Arc<CibouletteResourceType>,
    pub rel_opt: Ciboulette2PostgresMultiRelationships,
    pub values: Option<Vec<Ciboulette2SqlValue<'request>>>,
}

/// Extract informations concerning the main resource's one-to-many relationships
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Ciboulette2PostgresMultiRelationships {
    OneToMany(CibouletteRelationshipOneToManyOption),
    ManyToOne(CibouletteRelationshipOneToManyOption),
    ManyToMany(CibouletteRelationshipManyToManyOption),
}

impl Ciboulette2PostgresMultiRelationships {
    pub fn dest_resource(&self) -> &CibouletteResourceType {
        match self {
            Ciboulette2PostgresMultiRelationships::OneToMany(x) => x.many_table(),
            Ciboulette2PostgresMultiRelationships::ManyToOne(x) => x.many_table(),
            Ciboulette2PostgresMultiRelationships::ManyToMany(x) => x.bucket_resource(),
        }
    }

    pub fn dest_key(
        &self,
        main_type: &CibouletteResourceType,
    ) -> Result<ArcStr, CibouletteError> {
        match self {
            Ciboulette2PostgresMultiRelationships::OneToMany(x) => Ok(x.many_table_key().clone()),
            Ciboulette2PostgresMultiRelationships::ManyToOne(x) => Ok(x.many_table_key().clone()),
            Ciboulette2PostgresMultiRelationships::ManyToMany(x) => x.keys_for_type(main_type),
        }
    }
}

/// Extract data from a single relationship object
fn extract_many_to_one_relationships_from_ressource_identifiers<'request>(
    rel_ids: &'request CibouletteUpdateRelationship<'request>,
    rel_opt: &CibouletteRelationshipOneToManyOption,
) -> Result<Ciboulette2PostgresMainResourceInformations<'request>, Ciboulette2SqlError> {
    match rel_ids.value() {
        CibouletteOptionalData::Object(CibouletteResourceIdentifierSelector::One(rel_id)) => {
            Ok(Ciboulette2PostgresMainResourceInformations {
                insert_values: vec![(
                    rel_opt.many_table_key().clone(),
                    Ciboulette2SqlValue::from(rel_id.id()),
                )],
                single_relationships: vec![rel_opt.many_table_key().clone()],
            })
        }
        CibouletteOptionalData::Object(_) => {
            Err(Ciboulette2SqlError::MultiIdsForSingleRelationships)
        }
        CibouletteOptionalData::Null(x) if *x => Ok(Ciboulette2PostgresMainResourceInformations {
            insert_values: vec![(
                rel_opt.many_table_key().clone(),
                match rel_opt.many_table().id_type() {
                    CibouletteIdType::Text => Ciboulette2SqlValue::Text(None),
                    CibouletteIdType::Number => Ciboulette2SqlValue::Numeric(None),
                    CibouletteIdType::Uuid => Ciboulette2SqlValue::Uuid(None),
                },
            )],
            single_relationships: vec![rel_opt.many_table_key().clone()],
        }),
        CibouletteOptionalData::Null(_) => {
            Ok(Ciboulette2PostgresMainResourceInformations::default())
        }
    }
}

/// Extract one-to-many relationships
fn extract_relationships<'request>(
    buf: &mut Vec<Ciboulette2PostgresMainResourceRelationships<'request>>,
    relationships: Option<&'request BTreeMap<ArcStr, CibouletteRelationshipObject<'request>>>,
    type_: Arc<CibouletteResourceType>,
    type_to_alias: &str,
    opt: Ciboulette2PostgresMultiRelationships,
) {
    let relationships = match relationships {
        Some(x) => x,
        None => {
            buf.push(Ciboulette2PostgresMainResourceRelationships {
                type_,
                rel_opt: opt,
                values: None,
            });
            return;
        }
    };
    match relationships
        .get(type_to_alias)
        .map(|x| x.data())
        .unwrap_or(&CibouletteOptionalData::Null(false))
    {
        CibouletteOptionalData::Object(CibouletteResourceIdentifierSelector::One(rel_id)) => {
            buf.push(Ciboulette2PostgresMainResourceRelationships {
                type_,
                rel_opt: opt,
                values: Some(vec![Ciboulette2SqlValue::from(rel_id.id())]),
            });
        }
        CibouletteOptionalData::Object(CibouletteResourceIdentifierSelector::Many(rels_id)) => {
            buf.push(Ciboulette2PostgresMainResourceRelationships {
                type_,
                rel_opt: opt,
                values: Some(
                    rels_id
                        .iter()
                        .map(|x| Ciboulette2SqlValue::from(x.id()))
                        .collect(),
                ),
            });
        }
        CibouletteOptionalData::Null(x) if *x => {
            buf.push(Ciboulette2PostgresMainResourceRelationships {
                type_,
                rel_opt: opt,
                values: Some(vec![Ciboulette2SqlValue::Text(None)]), // FIXME Not always TEXT
            })
        }
        CibouletteOptionalData::Null(_) => buf.push(Ciboulette2PostgresMainResourceRelationships {
            type_,
            rel_opt: opt,
            values: None,
        }),
    }
}

/// Extract one-to-one relationship informations, without its values from the request
pub(crate) fn extract_fields_rel<'request>(
    store: &CibouletteStore,
    main_type: Arc<CibouletteResourceType>,
    rels: &'request CibouletteUpdateRelationship<'request>,
) -> Result<Ciboulette2PostgresMainResourceInformations<'request>, Ciboulette2SqlError> {
    let main_type_index = store
        .get_type_index(main_type.name())
        .ok_or_else(|| CibouletteError::UnknownType(main_type.name().to_string()))?;
    let rel_type_index = store
        .get_type_index(rels.type_().name())
        .ok_or_else(|| CibouletteError::UnknownType(rels.type_().name().to_string()))?;

    if let Some(rel_edge) = store.graph().find_edge(*main_type_index, *rel_type_index) {
        if let Some(rel) = store.graph().edge_weight(rel_edge) {
            match rel {
                CibouletteRelationshipOption::ManyToOne(opt) => {
                    return extract_many_to_one_relationships_from_ressource_identifiers(
                        &rels, &opt,
                    );
                }
                CibouletteRelationshipOption::OneToMany(_)
                | CibouletteRelationshipOption::ManyToMany(_) => {
                    return Err(Ciboulette2SqlError::UpdatingManyRelationships)
                }
            }
        }
    }
    Err(Ciboulette2SqlError::CibouletteError(
        CibouletteError::RelNotInGraph(
            main_type.name().to_string(),
            rels.type_().name().to_string(),
        ),
    ))
}

/// Extract one-to-one relationships informations, without their values from the request
pub(crate) fn extract_fields<'request>(
    store: &CibouletteStore,
    main_type: Arc<CibouletteResourceType>,
    relationships: Option<&'request BTreeMap<ArcStr, CibouletteRelationshipObject<'request>>>,
) -> Result<Vec<Ciboulette2PostgresMainResourceRelationships<'request>>, Ciboulette2SqlError> {
    let mut res: Vec<Ciboulette2PostgresMainResourceRelationships<'request>> = Vec::new(); // Vector in which the relationships queries will be stored

    let main_type_index = store
        .get_type_index(main_type.name())
        .ok_or_else(|| CibouletteError::UnknownType(main_type.name().to_string()))?;
    let mut walker = store
        .graph()
        .neighbors_directed(*main_type_index, petgraph::Direction::Incoming)
        .detach(); // Create a graph walker
    while let Some((edge_index, node_index)) = walker.next(&store.graph()) {
        // For each connect edge outgoing from the original node
        let edge_weight = store.graph().edge_weight(edge_index).unwrap(); //TODO unwrap // Get the edge weight
        let node_weight = store.graph().node_weight(node_index).unwrap(); //TODO unwrap // Get the node weight
        match &edge_weight {
            CibouletteRelationshipOption::OneToMany(opt) => {
                let type_to_alias = main_type.get_alias(node_weight.name().as_str())?; // Get the alias translation of that resource
                extract_relationships(
                    &mut res,
                    relationships,
                    node_weight.clone(),
                    type_to_alias,
                    Ciboulette2PostgresMultiRelationships::OneToMany(opt.clone()),
                );
            }
            CibouletteRelationshipOption::ManyToOne(opt) => {
                let type_to_alias = main_type.get_alias(node_weight.name().as_str())?; // Get the alias translation of that resource
                extract_relationships(
                    &mut res,
                    relationships,
                    node_weight.clone(),
                    type_to_alias,
                    Ciboulette2PostgresMultiRelationships::ManyToOne(opt.clone()),
                );
            }
            CibouletteRelationshipOption::ManyToMany(opt) => {
                let type_to_alias = main_type.get_alias(node_weight.name().as_str())?; // Get the alias translation of that resource
                extract_relationships(
                    &mut res,
                    relationships,
                    node_weight.clone(),
                    type_to_alias,
                    Ciboulette2PostgresMultiRelationships::ManyToMany(opt.clone()),
                );
            }
        }
    }
    Ok(res)
}

/// Get the multi relationships informations of a resource
pub(crate) fn get_resource_multi_rels<'request>(
    store: &CibouletteStore,
    main_type: Arc<CibouletteResourceType>,
) -> Result<Vec<Ciboulette2PostgresMainResourceRelationships<'request>>, Ciboulette2SqlError> {
    let mut res: Vec<Ciboulette2PostgresMainResourceRelationships<'request>> = Vec::new(); // Vector in which the relationships queries will be stored

    let main_type_index = store
        .get_type_index(main_type.name())
        .ok_or_else(|| CibouletteError::UnknownType(main_type.name().to_string()))?;
    let mut walker = store
        .graph()
        .neighbors_directed(*main_type_index, petgraph::Direction::Incoming)
        .detach(); // Create a graph walker
    while let Some((edge_index, node_index)) = walker.next(&store.graph()) {
        // For each connect edge outgoing from the original node
        let edge_weight = store.graph().edge_weight(edge_index).unwrap(); //TODO unwrap // Get the edge weight
        match edge_weight {
            CibouletteRelationshipOption::OneToMany(opt)
                if opt.part_of_many_to_many().is_none() =>
            {
                let node_weight = store.graph().node_weight(node_index).unwrap(); //TODO unwrap // Get the node weight
                res.push(Ciboulette2PostgresMainResourceRelationships {
                    type_: node_weight.clone(),
                    rel_opt: Ciboulette2PostgresMultiRelationships::OneToMany(opt.clone()),
                    values: None,
                });
            }
            CibouletteRelationshipOption::ManyToMany(opt) => {
                let node_weight = store.graph().node_weight(node_index).unwrap(); //TODO unwrap // Get the node weight
                res.push(Ciboulette2PostgresMainResourceRelationships {
                    type_: node_weight.clone(),
                    rel_opt: Ciboulette2PostgresMultiRelationships::ManyToMany(opt.clone()),
                    values: None,
                });
            }
            _ => continue,
        }
    }
    Ok(res)
}
