use super::*;

// pub mod main;
// pub mod relationships;
use sqlx::{Acquire, Executor, Transaction};
// pub async fn ciboulette2sql<
//     'a,
//     E: sqlx::Executor<'a, Database = sqlx::Postgres> + sqlx::Acquire<'a, Database = sqlx::Postgres>,
// >(
//     conn: E,
//     store: &'a CibouletteStore,
//     req: &'a CibouletteCreateRequest<'a>,
// ) -> Result<(), Ciboulette2SqlError> {
//     let (main_query, params) = main::gen_query_insert_main(store, req)?;
//     let step1_query = sqlx::query_as_with(step1_req.as_str(), step1_params);

//     let mut transactions: Transaction<sqlx::Postgres> = conn.begin().await?;

//     println!("Running {:#?}", step1_req);

//     let step1_res: Ciboulette2SqlResultWithId = step1_query.fetch_one(&mut transactions).await?;

//     println!("{:#?}", step1_res);
//     Ok(())
// }
