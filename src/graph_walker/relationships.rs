use super::*;
use main::Ciboulette2PostgresMainResourceInformations;
/// Extract informations concerning the main resource's one-to-many relationships
#[derive(Clone, Debug, Getters)]
#[getset(get = "pub")]
pub(crate) struct Ciboulette2PostgresMainResourceRelationships<'a> {
    pub type_: &'a CibouletteResourceType<'a>,
    pub rel_opt: Ciboulette2PostgresMultiRelationships<'a>,
    pub values: Option<Vec<Ciboulette2SqlValue<'a>>>,
}

/// Extract informations concerning the main resource's one-to-many relationships
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Ciboulette2PostgresMultiRelationships<'a> {
    OneToMany(&'a CibouletteRelationshipOneToManyOption<'a>),
    ManyToMany(&'a CibouletteRelationshipManyToManyOption<'a>),
}

impl<'a> Ciboulette2PostgresMultiRelationships<'a> {
    pub fn dest_resource(&self) -> &CibouletteResourceType<'a> {
        match self {
            Ciboulette2PostgresMultiRelationships::OneToMany(x) => x.many_table(),
            Ciboulette2PostgresMultiRelationships::ManyToMany(x) => x.bucket_resource(),
        }
    }

    pub fn dest_key(
        &self,
        main_type: &CibouletteResourceType<'a>,
    ) -> Result<&str, CibouletteError> {
        match self {
            Ciboulette2PostgresMultiRelationships::OneToMany(x) => Ok(x.many_table_key().as_str()),
            Ciboulette2PostgresMultiRelationships::ManyToMany(x) => x.keys_for_type(main_type),
        }
    }
}

/// Extract data from a single relationship object
fn extract_single_relationships_from_ressource_identifiers<'a>(
    rel_ids: &'a CibouletteUpdateRelationship<'a>,
    rel_opt: &'a CibouletteRelationshipOneToOneOption,
) -> Result<Ciboulette2PostgresMainResourceInformations<'a>, Ciboulette2SqlError> {
    match rel_ids.value() {
        CibouletteOptionalData::Object(CibouletteResourceIdentifierSelector::One(rel_id)) => {
            Ok(Ciboulette2PostgresMainResourceInformations {
                insert_values: vec![(
                    rel_opt.key().as_str(),
                    Ciboulette2SqlValue::from(rel_id.id()),
                )],
                single_relationships: vec![rel_opt.key().as_str()],
            })
        }
        CibouletteOptionalData::Object(_) => {
            Err(Ciboulette2SqlError::MultiIdsForSingleRelationships)
        }
        CibouletteOptionalData::Null(x) if *x => Ok(Ciboulette2PostgresMainResourceInformations {
            insert_values: vec![(
                rel_opt.key().as_str(),
                match rel_opt.id_type() {
                    CibouletteIdType::Text => Ciboulette2SqlValue::Text(None),
                    CibouletteIdType::Number => Ciboulette2SqlValue::Numeric(None),
                    CibouletteIdType::Uuid => Ciboulette2SqlValue::Uuid(None),
                },
            )],
            single_relationships: vec![rel_opt.key().as_str()],
        }),
        CibouletteOptionalData::Null(_) => {
            Ok(Ciboulette2PostgresMainResourceInformations::default())
        }
    }
}

/// Extract one-to-many relationships
fn extract_relationships<'a>(
    buf: &mut Vec<Ciboulette2PostgresMainResourceRelationships<'a>>,
    relationships: Option<&'a BTreeMap<Cow<'a, str>, CibouletteRelationshipObject<'a>>>,
    type_: &'a CibouletteResourceType<'a>,
    type_to_alias: &'a str,
    opt: Ciboulette2PostgresMultiRelationships<'a>,
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
pub(crate) fn extract_fields_rel<'a>(
    store: &'a CibouletteStore<'a>,
    main_type: &'a CibouletteResourceType<'a>,
    rels: &'a CibouletteUpdateRelationship<'a>,
) -> Result<Ciboulette2PostgresMainResourceInformations<'a>, Ciboulette2SqlError> {
    let main_type_index = store
        .get_type_index(main_type.name())
        .ok_or_else(|| CibouletteError::UnknownType(main_type.name().to_string()))?;
    let rel_type_index = store
        .get_type_index(rels.type_().name())
        .ok_or_else(|| CibouletteError::UnknownType(rels.type_().name().to_string()))?;

    for rel in store
        .graph()
        .edges_connecting(*main_type_index, *rel_type_index)
    {
        match rel.weight() {
            CibouletteRelationshipOption::OneToOne(opt) => {
                return Ok(extract_single_relationships_from_ressource_identifiers(
                    &rels, &opt,
                )?)
            }
            CibouletteRelationshipOption::OneToMany(_)
            | CibouletteRelationshipOption::ManyToOne(_)
            | CibouletteRelationshipOption::ManyToMany(_) => {
                return Err(Ciboulette2SqlError::UpdatingManyRelationships)
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
pub(crate) fn extract_fields<'a>(
    store: &'a CibouletteStore<'a>,
    main_type: &'a CibouletteResourceType<'a>,
    relationships: Option<&'a BTreeMap<Cow<'a, str>, CibouletteRelationshipObject<'a>>>,
) -> Result<Vec<Ciboulette2PostgresMainResourceRelationships<'a>>, Ciboulette2SqlError> {
    let mut res: Vec<Ciboulette2PostgresMainResourceRelationships<'a>> = Vec::new(); // Vector in which the relationships queries will be stored

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
            CibouletteRelationshipOption::ManyToOne(opt)
            | CibouletteRelationshipOption::OneToMany(opt) => {
                let type_to_alias: &String = main_type.get_alias(node_weight.name().as_str())?; // Get the alias translation of that resource
                extract_relationships(
                    &mut res,
                    relationships,
                    node_weight,
                    type_to_alias,
                    Ciboulette2PostgresMultiRelationships::OneToMany(opt),
                );
            }
            CibouletteRelationshipOption::ManyToMany(opt) => {
                let type_to_alias: &String = main_type.get_alias(node_weight.name().as_str())?; // Get the alias translation of that resource
                extract_relationships(
                    &mut res,
                    relationships,
                    node_weight,
                    type_to_alias,
                    Ciboulette2PostgresMultiRelationships::ManyToMany(opt),
                );
            }
            _ => continue,
        }
    }
    Ok(res)
}

/// Get the multi relationships informations of a resource
pub(crate) fn get_resource_multi_rels<'a>(
    store: &'a CibouletteStore<'a>,
    main_type: &'a CibouletteResourceType<'a>,
) -> Result<Vec<Ciboulette2PostgresMainResourceRelationships<'a>>, Ciboulette2SqlError> {
    let mut res: Vec<Ciboulette2PostgresMainResourceRelationships<'a>> = Vec::new(); // Vector in which the relationships queries will be stored

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
                    type_: node_weight,
                    rel_opt: Ciboulette2PostgresMultiRelationships::OneToMany(&opt),
                    values: None,
                });
            }
            CibouletteRelationshipOption::ManyToMany(opt) => {
                let node_weight = store.graph().node_weight(node_index).unwrap(); //TODO unwrap // Get the node weight
                res.push(Ciboulette2PostgresMainResourceRelationships {
                    type_: node_weight,
                    rel_opt: Ciboulette2PostgresMultiRelationships::ManyToMany(&opt),
                    values: None,
                });
            }
            _ => continue,
        }
    }
    Ok(res)
}
