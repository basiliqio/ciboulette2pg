use super::*;

fn extract_relationships<'a>(
    buf: &mut Vec<(
        &'a CibouletteResourceType<'a>,
        &'a CibouletteRelationshipBucket<'a>,
        Vec<Ciboulette2SqlValue<'a>>,
    )>,
    relationships: &'a BTreeMap<Cow<'a, str>, CibouletteRelationshipObject<'a>>,
    type_: &'a CibouletteResourceType<'a>,
    type_to_alias: &'a str,
    opt: &'a CibouletteRelationshipBucket<'a>,
) -> Result<(), Ciboulette2SqlError> {
    match relationships
        .get(type_to_alias)
        .and_then(|x| x.data().as_ref())
    {
        Some(CibouletteResourceIdentifierSelector::One(rel_id)) => {
            buf.push((
                type_,
                opt,
                vec![Ciboulette2SqlValue::Text(Some(Cow::Borrowed(rel_id.id())))],
            ));
            Ok(())
        }
        Some(CibouletteResourceIdentifierSelector::Many(rels_id)) => {
            buf.push((
                type_,
                opt,
                rels_id
                    .iter()
                    .map(|x| Ciboulette2SqlValue::Text(Some(Cow::Borrowed(x.id()))))
                    .collect(),
            ));
            Ok(())
        }
        None => Ok(()),
    }
}

pub fn gen_query_insert<'a>(
    store: &'a CibouletteStore<'a>,
    req: &'a CibouletteCreateRequest<'a>,
) -> Result<
    Vec<(
        &'a CibouletteResourceType<'a>,
        &'a CibouletteRelationshipBucket<'a>,
        Vec<Ciboulette2SqlValue<'a>>,
    )>,
    Ciboulette2SqlError,
> {
    let mut res: Vec<(
        &'a CibouletteResourceType<'a>,
        &'a CibouletteRelationshipBucket<'a>,
        Vec<Ciboulette2SqlValue<'a>>,
    )> = Vec::new(); // Vector in which the relationships queries will be stored

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
        match edge_weight {
            CibouletteRelationshipOption::ManyDirect(opt) => {
                let node_weight = store.graph().node_weight(node_index).unwrap(); //TODO unwrap // Get the node weight
                let type_to_alias: &String = main_type.get_alias(node_weight.name().as_str())?; // Get the alias translation of that resource
                extract_relationships(
                    &mut res,
                    req.data().relationships(),
                    node_weight,
                    type_to_alias,
                    &opt,
                )?;
            }
            _ => (),
        }
    }
    Ok(res)
}
