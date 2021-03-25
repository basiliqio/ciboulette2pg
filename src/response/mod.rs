use super::*;
pub mod response_type;
use graph_walker::main;
use itertools::Itertools;

// pub fn gen_response<'a, T, I>(
//     ciboulette_store: &'a CibouletteStore<'a>,
//     ciboulette_table_store: &'a Ciboulette2PostgresTableStore<'a>,
//     request: &'a T,
//     rows: I,
// ) -> Result<CibouletteOutboundRequest<'a>, Ciboulette2SqlError>
// where
//     T: CibouletteInboundRequestCommons<'a>,
//     I: Itertools<Item = Ciboulette2PostgresRow<'a>>,
// {
//     let rows = rows.group_by(|x| x.type_().clone());
//     let main_data: BTreeMap<
//         &CibouletteResourceIdentifier<'a>,
//         CibouletteResource<'a, CibouletteResourceIdentifier<'a>>,
//     > = BTreeMap::new();
//     let relationships: BTreeMap<Cow<'a, str>, CibouletteRelationshipObject<'a>> = BTreeMap::new();
//     let included: Vec<CibouletteResource<CibouletteResourceIdentifier>> = Vec::new();

//     for (key, group) in rows.into_iter() {
// 		let identifier = CibouletteResourceIdentifier::new(
// 			CibouletteId::Text(row.id),
// 			row.type_,
// 			serde_json::Value::Null,
// 		); // TODO meta
//         match key == request.path().main_type().name().as_str() {
//             true => {
//                 for row in group.into_iter() {
//                     match request.expected_response_type() {
//                         CibouletteResponseRequiredType::Object(_) => {
//                             let resource = CibouletteResource {
//                                 identifier: identifier.clone(),
//                                 attributes: Some(row.data),
//                                 ..Default::default()
//                             };
//                             if let Some(_) = main_data.insert(resource.identifier(), resource) {
//                                 todo!()
//                             }
//                         }
//                         CibouletteResponseRequiredType::Id(_) => {
//                             let resource = CibouletteResource {
//                                 identifier: identifier.clone(),
//                                 ..Default::default()
//                             };
//                             if let Some(_) = main_data.insert(resource.identifier(), resource) {
//                                 todo!()
//                             }
//                         }
//                         CibouletteResponseRequiredType::None => {}
//                     }
//                 }
//             }
//             false => {
// 				for row in group.into_iter() {
//                     match request.expected_response_type() {
//                         CibouletteResponseRequiredType::Object(_) => {
//                             let resource = CibouletteResource {
//                                 identifier: identifier.clone(),
//                                 attributes: Some(row.data),
//                                 ..Default::default()
//                             };
//                             if let Some(_) = main_data.insert(resource.identifier(), resource) {
//                                 todo!()
//                             }
//                         }
//                         CibouletteResponseRequiredType::Id(_) => {
//                             let resource = CibouletteResource {
//                                 identifier: identifier.clone(),
//                                 ..Default::default()
//                             };
//                             if let Some(_) = main_data.insert(resource.identifier(), resource) {
//                                 todo!()
//                             }
//                         }
//                         CibouletteResponseRequiredType::None => {}
//                     }
//                 }
// 			},
//         }
//     }
//     todo!();
// }
