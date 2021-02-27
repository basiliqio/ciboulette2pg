use super::*;

fn insert_relationships<'a>(
	relationships: &'a BTreeMap<Cow<'a, str>, CibouletteRelationshipObject<'a>>,
	type_to_name: &'a str,
	type_to_alias: &'a str,
	opt: &'a CibouletteRelationshipBucket,
	params: Vec<quaint::Value<'a>>
) -> Result<Vec<(String, Vec<quaint::ast::Value<'a>>)>, Ciboulette2SqlError> {
	println!("{:#?}", relationships
	.get(type_to_alias));
	match relationships
		.get(type_to_alias)
		.and_then(|x| x.data().as_ref())
	{
		Some(CibouletteResourceIdentifierSelector::One(rel_id)) => {
			let fields: &[&str; 2] = &[opt.from().as_str(), opt.to().as_str()];
			let (query, params) = Postgres::build(
				Insert::single_into(opt.resource().name())
					.value(fields[0], main_id)
					.value(fields[1], rel_id.id().to_string())
					.build()
					.returning(fields),
			)?;
			Ok(vec![crate::utils::json_wrap_with_id_and_type(
				query,
				params,
				type_to_name,
			)])
		}
		Some(CibouletteResourceIdentifierSelector::Many(rels_id)) => {
			let mut values: Vec<&str> = Vec::with_capacity(rels_id.len());
			let mut res: Vec<(String, Vec<quaint::ast::Value<'a>>)> =
				Vec::with_capacity(rels_id.len());
			let fields: &[&str; 2] = &[opt.from().as_str(), opt.to().as_str()];
			let insert_stmt = Insert::multi_into(opt.resource().name(), fields);
			for rel_id in rels_id.iter() {
				values.push([main_id, rel_id.id().as_ref()].to_vec());
			}
			let (query, params) =
				Postgres::build(insert_stmt.values(values).build().returning(fields))?;
			res.push(crate::utils::json_wrap_with_id_and_type(
				query,
				params,
				type_to_name,
			));
			Ok(res)
		}
		None => Ok(Vec::new()),
	}
}

pub fn gen_query_insert_relationships<'a>(
	store: &'a CibouletteStore,
	req: &'a CibouletteCreateRequest<'a>,
	params: Vec<quaint::Value<'a>>
) -> Result<Vec<(String, Vec<quaint::ast::Value<'a>>)>, Ciboulette2SqlError> {
	let mut res: Vec<(String, Vec<quaint::ast::Value<'_>>)> = Vec::new(); // Vector in which the relationships queries will be stored

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
				println!("{:#?}", main_type.relationships_type_to_alias());
				let type_to_alias: &String = main_type.get_alias(node_weight.name().as_str())?; // Get the alias translation of that resource
				res.append(&mut insert_relationships(
					req.data().relationships(),
					node_weight.name().as_str(),
					type_to_alias,
					&opt,
					params,
				)?);
			}
			_ => (),
		}
	}
	Ok(res)
}
