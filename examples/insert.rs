// use ciboulette::*;
// use ciboulette2sql::*;
// use ciboulette_test_helper::*;
// use std::convert::TryInto;
// use url::Url;

// const BODY: &'static str = r#"
// {
//   "data":
//   {
//     "type": "peoples",
//     "attributes":
// 	{
//       "first-name": "Hello",
//       "last-name": "World"
//     },
// 	"relationships":
// 	{
// 	  "articles":
// 	  {
// 		  "data":
// 		  {
// 			"type": "articles",
// 			"id": "9285eda2-73b0-4e02-a5d9-6ec31cf22692"
// 		  }
// 	  }
// 	}
//   }
// }
// "#;
// use quaint::{
//     prelude::*,
//     visitor::{Postgres, Visitor},
// };
// #[tokio::main]
// async fn main() {
//     let req_url = Url::parse("http://localhost:8080/peoples").unwrap();
//     let store = gen_bag();
//     let req: CibouletteCreateRequest =
//         CibouletteRequestBuilder::new(CibouletteIntention::Create, &req_url, &Some(BODY))
//             .build(&store)
//             .unwrap()
//             .try_into()
//             .unwrap();
//     // let pool =
//     //     sqlx::Pool::<sqlx::Postgres>::connect(std::env::var("DATABASE_URL").unwrap().as_str())
//     //         .await
//     //         .unwrap();
//     println!(
//         "Main : {:#?}",
//         ciboulette2sql::creation::main::gen_query_main(&store, &req).unwrap()
//     );

//     println!(
//         "Rel : {:#?}",
//         ciboulette2sql::creation::relationships::gen_query_relationships(
//             &store, &req, "MAIN_ID"
//         )
//         .unwrap()
//     );

//     let conditions = "world"
//         .equals("meow")
//         .and("age".less_than(10))
//         .and("paw".equals("warm"));

//     let query = Select::from_table("naukio").so_that(conditions);
//     let (sql_str, params) = Postgres::build(query).unwrap();
//     println!("{}", sql_str);
// }

fn main() {}
