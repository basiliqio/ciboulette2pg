use super::*;
use main::Ciboulette2PostgresMain;
//
// 	&'a CibouletteResourceType<'a>,
//	&'a CibouletteRelationshipBucket<'a>,
//	Vec<Ciboulette2SqlValue<'a>>,
//
//

#[derive(Clone, Debug, Getters)]
#[getset(get = "pub")]
pub struct Ciboulette2PostgresRelationships<'a> {
    pub type_: &'a CibouletteResourceType<'a>,
    pub bucket: &'a CibouletteRelationshipBucket<'a>,
    pub values: Option<Vec<Ciboulette2SqlValue<'a>>>,
}

pub enum Ciboulette2PostgresRelationshipsResultType<'a> {
    Single(Ciboulette2PostgresMain<'a>),
    Multi(Ciboulette2PostgresRelationships<'a>),
}

fn extract_multi_relationships_from_ressource_identifiers<'a>(
    rel_ids: &'a CibouletteResourceIdentifierSelector<'a>,
    rel_opt: &'a CibouletteRelationshipBucket<'a>,
    rel_type: &'a CibouletteResourceType<'a>,
) -> Result<Ciboulette2PostgresRelationships<'a>, Ciboulette2SqlError> {
    match rel_ids {
        CibouletteResourceIdentifierSelector::One(rel_id) => Ok(Ciboulette2PostgresRelationships {
            type_: rel_type,
            bucket: rel_opt,
            values: Some(vec![Ciboulette2SqlValue::Text(Some(Cow::Borrowed(
                rel_id.id(),
            )))]),
        }),
        CibouletteResourceIdentifierSelector::Many(rels_id) => {
            Ok(Ciboulette2PostgresRelationships {
                type_: rel_type,
                bucket: rel_opt,
                values: Some(
                    rels_id
                        .iter()
                        .map(|x| Ciboulette2SqlValue::Text(Some(Cow::Borrowed(x.id()))))
                        .collect(),
                ),
            })
        }
    }
}

fn extract_single_relationships_from_ressource_identifiers<'a>(
    rel_ids: &'a CibouletteUpdateRelationship<'a>,
    rel_opt: &'a CibouletteRelationshipOneToOneOption,
) -> Result<Ciboulette2PostgresMain<'a>, Ciboulette2SqlError> {
    match rel_ids.value() {
        CibouletteOptionalData::Object(CibouletteResourceIdentifierSelector::One(rel_id)) => {
            Ok(Ciboulette2PostgresMain {
                insert_values: vec![(
                    rel_opt.key().as_str(),
                    Ciboulette2SqlValue::Text(Some(Cow::Borrowed(rel_id.id()))), //TODO, not always text
                )],
                single_relationships: vec![rel_opt.key().as_str()],
            })
        }
        CibouletteOptionalData::Object(_) => Err(Ciboulette2SqlError::UpdatingManyRelationships),
        CibouletteOptionalData::Null(x) if *x => Ok(Ciboulette2PostgresMain {
            insert_values: vec![(
                rel_opt.key().as_str(),
                Ciboulette2SqlValue::Text(None), //TODO, not always text
            )],
            single_relationships: vec![rel_opt.key().as_str()],
        }),
        CibouletteOptionalData::Null(_) => Ok(Ciboulette2PostgresMain::default()),
    }
}

fn extract_relationships<'a>(
    buf: &mut Vec<Ciboulette2PostgresRelationships<'a>>,
    relationships: &'a BTreeMap<Cow<'a, str>, CibouletteRelationshipObject<'a>>,
    type_: &'a CibouletteResourceType<'a>,
    type_to_alias: &'a str,
    opt: &'a CibouletteRelationshipBucket<'a>,
) {
    match relationships
        .get(type_to_alias)
        .map(|x| x.data())
        .unwrap_or(&CibouletteOptionalData::Null(false))
    {
        CibouletteOptionalData::Object(CibouletteResourceIdentifierSelector::One(rel_id)) => {
            buf.push(Ciboulette2PostgresRelationships {
                type_,
                bucket: opt,
                values: Some(vec![Ciboulette2SqlValue::Text(Some(Cow::Borrowed(
                    rel_id.id(),
                )))]),
            });
        }
        CibouletteOptionalData::Object(CibouletteResourceIdentifierSelector::Many(rels_id)) => {
            buf.push(Ciboulette2PostgresRelationships {
                type_,
                bucket: opt,
                values: Some(
                    rels_id
                        .iter()
                        .map(|x| Ciboulette2SqlValue::Text(Some(Cow::Borrowed(x.id()))))
                        .collect(),
                ),
            });
        }
        CibouletteOptionalData::Null(x) if *x => buf.push(Ciboulette2PostgresRelationships {
            type_,
            bucket: opt,
            values: Some(vec![Ciboulette2SqlValue::Text(None)]),
        }),
        CibouletteOptionalData::Null(_) => buf.push(Ciboulette2PostgresRelationships {
            type_,
            bucket: opt,
            values: None,
        }),
    }
}

pub fn gen_query_rel<'a>(
    store: &'a CibouletteStore<'a>,
    main_type: &'a CibouletteResourceType<'a>,
    rels: &'a CibouletteUpdateRelationship<'a>,
) -> Result<Ciboulette2PostgresMain<'a>, Ciboulette2SqlError> {
    let main_type_index = store
        .get_type_index(main_type.name())
        .ok_or_else(|| CibouletteError::UnknownType(main_type.name().to_string()))?;
    let rel_type_index = store
        .get_type_index(rels.type_().name())
        .ok_or_else(|| CibouletteError::UnknownType(rels.type_().name().to_string()))?;

    for rel in store
        .graph()
        .edges_connecting(*rel_type_index, *main_type_index)
    {
        match rel.weight() {
            CibouletteRelationshipOption::One(opt) => {
                return Ok(extract_single_relationships_from_ressource_identifiers(
                    &rels, &opt,
                )?)
            }
            CibouletteRelationshipOption::ManyDirect(opt) => {
                return Err(Ciboulette2SqlError::UpdatingManyRelationships)
            }
            CibouletteRelationshipOption::Many(_) => continue,
        }
    }
    Err(Ciboulette2SqlError::CibouletteError(
        CibouletteError::RelNotInGraph(
            main_type.name().to_string(),
            rels.type_().name().to_string(),
        ),
    ))
}

pub fn gen_query<'a>(
    store: &'a CibouletteStore<'a>,
    main_type: &'a CibouletteResourceType<'a>,
    relationships: &'a BTreeMap<Cow<'a, str>, CibouletteRelationshipObject<'a>>,
) -> Result<Vec<Ciboulette2PostgresRelationships<'a>>, Ciboulette2SqlError> {
    let mut res: Vec<Ciboulette2PostgresRelationships<'a>> = Vec::new(); // Vector in which the relationships queries will be stored

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
        if let CibouletteRelationshipOption::ManyDirect(opt) = edge_weight {
            let node_weight = store.graph().node_weight(node_index).unwrap(); //TODO unwrap // Get the node weight
            let type_to_alias: &String = main_type.get_alias(node_weight.name().as_str())?; // Get the alias translation of that resource
            extract_relationships(&mut res, relationships, node_weight, type_to_alias, &opt);
        }
    }
    Ok(res)
}
