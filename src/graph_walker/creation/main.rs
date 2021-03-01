use super::*;
use std::convert::TryFrom;

fn check_single_relationships<'a>(
    relationships: &'a BTreeMap<Cow<'a, str>, CibouletteRelationshipObject<'a>>,
    from_type_: &'a CibouletteResourceType,
    to_type_: &'a CibouletteResourceType,
    to_type_alias: &'a str,
    opt: &'a CibouletteRelationshipOneToOneOption,
) -> Result<Option<(&'a str, Ciboulette2SqlValue<'a>)>, Ciboulette2SqlError> {
    match relationships.get(to_type_alias) {
        Some(rel_obj) => match rel_obj.data() {
            Some(CibouletteResourceIdentifierSelector::One(rel_id)) => Ok(Some((
                opt.key().as_str(),
                Ciboulette2SqlValue::Text(Some(Cow::Borrowed(rel_id.id()))),
            ))),
            Some(CibouletteResourceIdentifierSelector::Many(_)) => {
                return Err(Ciboulette2SqlError::RequiredSingleRelationship(
                    from_type_.name().to_string(),
                ));
            }
            None => {
                if !opt.optional() {
                    return Err(Ciboulette2SqlError::MissingRelationship(
                        from_type_.name().to_string(),
                        to_type_.name().to_string(),
                    ));
                }
                Ok(None)
            }
        },
        None => {
            if !opt.optional() {
                return Err(Ciboulette2SqlError::MissingRelationship(
                    from_type_.name().to_string(),
                    to_type_.name().to_string(),
                ));
            }
            Ok(None)
        }
    }
}

pub fn fill_attributes<'a>(
    args: &mut Vec<(&'a str, Ciboulette2SqlValue<'a>)>,
    obj: &'a Option<MessyJsonObjectValue<'a>>,
) -> Result<(), Ciboulette2SqlError> {
    if let Some(obj) = obj {
        for (k, v) in obj.iter() {
            // Iterate over every attribute
            args.push((k, Ciboulette2SqlValue::try_from(v)?));
        }
    }
    Ok(())
}

pub fn gen_query_insert<'a>(
    store: &'a CibouletteStore,
    req: &'a CibouletteCreateRequest<'a>,
) -> Result<Vec<(&'a str, Ciboulette2SqlValue<'a>)>, Ciboulette2SqlError> {
    let mut res: Vec<(&'a str, Ciboulette2SqlValue<'a>)> = Vec::with_capacity(128);
    let main_type = req.path().main_type();
    let main_type_index = store
        .get_type_index(main_type.name())
        .ok_or_else(|| CibouletteError::UnknownType(main_type.name().to_string()))?;

    fill_attributes(&mut res, &req.data().attributes())?;
    let mut walker = store
        .graph()
        .neighbors_directed(*main_type_index, petgraph::Direction::Outgoing)
        .detach(); // Create a graph walker
    while let Some((edge_index, node_index)) = walker.next(&store.graph()) {
        // For each connect edge outgoing from the original node
        if let CibouletteRelationshipOption::One(opt) =
            store.graph().edge_weight(edge_index).unwrap()
        // Get the edge weight
        {
            let node_weight = store.graph().node_weight(node_index).unwrap(); // Get the node weight
            let alias: &String = main_type.get_alias(node_weight.name().as_str())?; // Get the alias translation of that resource
            if let Some(v) = check_single_relationships(
                &req.data().relationships(),
                &main_type,
                &node_weight,
                alias,
                opt,
            )? {
                res.push(v); // Insert the relationship
            }
        }
    }
    Ok(res)
}
