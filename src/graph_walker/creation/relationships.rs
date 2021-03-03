use super::*;

//
// 	&'a CibouletteResourceType<'a>,
//	&'a CibouletteRelationshipBucket<'a>,
//	Vec<Ciboulette2SqlValue<'a>>,
//
//

#[derive(Clone, Debug, Getters)]
#[getset(get = "pub")]
pub struct Ciboulette2PostgresRelationshipsInsert<'a> {
    pub type_: &'a CibouletteResourceType<'a>,
    pub bucket: &'a CibouletteRelationshipBucket<'a>,
    pub values: Option<Vec<Ciboulette2SqlValue<'a>>>,
}

fn extract_relationships<'a>(
    buf: &mut Vec<Ciboulette2PostgresRelationshipsInsert<'a>>,
    relationships: &'a BTreeMap<Cow<'a, str>, CibouletteRelationshipObject<'a>>,
    type_: &'a CibouletteResourceType<'a>,
    type_to_alias: &'a str,
    opt: &'a CibouletteRelationshipBucket<'a>,
) {
    match relationships
        .get(type_to_alias)
        .and_then(|x| x.data().as_ref())
    {
        Some(CibouletteResourceIdentifierSelector::One(rel_id)) => {
            buf.push(Ciboulette2PostgresRelationshipsInsert {
                type_,
                bucket: opt,
                values: Some(vec![Ciboulette2SqlValue::Text(Some(Cow::Borrowed(
                    rel_id.id(),
                )))]),
            });
        }
        Some(CibouletteResourceIdentifierSelector::Many(rels_id)) => {
            buf.push(Ciboulette2PostgresRelationshipsInsert {
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
        None => buf.push(Ciboulette2PostgresRelationshipsInsert {
            type_,
            bucket: opt,
            values: None,
        }),
    }
}

pub fn gen_query_insert<'a>(
    store: &'a CibouletteStore<'a>,
    req: &'a CibouletteCreateRequest<'a>,
) -> Result<Vec<Ciboulette2PostgresRelationshipsInsert<'a>>, Ciboulette2SqlError> {
    let mut res: Vec<Ciboulette2PostgresRelationshipsInsert<'a>> = Vec::new(); // Vector in which the relationships queries will be stored

    let main_type = req.path().main_type();
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
            extract_relationships(
                &mut res,
                req.data().relationships(),
                node_weight,
                type_to_alias,
                &opt,
            );
        }
    }
    Ok(res)
}
