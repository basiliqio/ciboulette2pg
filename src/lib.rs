//! # Introduction
//!
//! **_Ciboulette2Pg_** is an library that execute [_Ciboulette_](ciboulette) requests as _Postgres_ queries and convert back the
//! result to [_Ciboulette_](ciboulette) responses.
//!
//! It strive to always execute [_Ciboulette_](ciboulette) requests as a single _Postgres_ query reducing latency.
//!
//! It support sparse fields, sorting and including related objects.
//!
//! ## High level operations
//!
//! ### Query structure
//!
//! When building a query, this library will make heavy use of [CTE](https://www.postgresql.org/docs/13/queries-with.html)s.
//! It allows the query to reference itself, having multiple sub-queries that perform different tasks.
//!
//! All the query return data in the same way.
//!
//! | Key            | Description                                                                                                                                             |
//! |----------------|---------------------------------------------------------------------------------------------------------------------------------------------------------|
//! | `id`           | The identifier of the resource, in TEXT format.                                                                                                         |
//! | `type`         | The type of the resource, in TEXT format.<br>If this row handles a relationship, this will be the relationship chain (i.e. `peoples.articles.comments`) |
//! | `data`         | Optional, JSON packed data object containing the attributes of the resource                                                                             |
//! | `related_id`   | The `id` of the resource it relates to                                                                                                                  |
//! | `related_type` | The `type` of the resource it relates to. If it relates to another relationship of the main data, it should contains the relationship chain             |
//!
//! The first [CTE](https://www.postgresql.org/docs/13/queries-with.html) will be the one applying the action of the request.
//! (i.e. For a create request, the first [CTE](https://www.postgresql.org/docs/13/queries-with.html) will perfom the `INSERT`).
//! Depending on the request type, a second [CTE](https://www.postgresql.org/docs/13/queries-with.html) will be used to select
//! the data modified by the first [CTE](https://www.postgresql.org/docs/13/queries-with.html).
//! After that, all the required (either included or required for sorting) relationships will be used.
//! Finally, depending on the sorting requirement, a final main [CTE](https://www.postgresql.org/docs/13/queries-with.html)
//! will be inserted to sort the main data.
//!
//! ### Query Response
//!
//! At the end of the query, all the necessary will be `UNION`ed together to form the response.
//!
//! All the [CTE](https://www.postgresql.org/docs/13/queries-with.html) have all the same commons keys as describe below,
//! some have additional columns for sorting or linking but these won't be included in the response.
//!
//! One can think of the response like the following schema in which different [CTE](https://www.postgresql.org/docs/13/queries-with.html)
//! contributing to the final reponse beeing built.
//!
//! ```ascii
//!                                                    ┌────────┐
//!                                                    │ Origin │
//!                                                    │ Table  │
//!                      ┌───┬─────────────────┬───────┴──────┬─┴─────────┬────────────┐
//!        Columns   ──► │ id│type             │     data     │related_id │related_type│
//!                      ├───┼─────────────────┼─────┬────────┴────┬──────┼────────────┤
//!                      │   │                 │     │cte_peoples_0│      │            │
//!      Action CTE  ──► │   │                 │     └────────┬────┘      │            │
//!                      │ 01│peoples          │   {<json>}   │   NULL    │ NULL       │
//!                      ├───┼─────────────────┼─────┬────────┴─────┬─────┼────────────┤
//!                      │   │                 │     │cte_articles_1│     │            │
//!                      │   │                 │     └────────┬─────┘     │            │
//!     Relationship ──► │ 42│articles         │   {<json>}   │    01     │ peoples    │
//!      "articles"      │ 43│articles         │   {<json>}   │    01     │ peoples    │
//!                      │ 44│articles         │   {<json>}   │    01     │ peoples    │
//!                      ├───┼─────────────────┼──┬───────────┴────────┬──┼────────────┤
//!                      │   │                 │  │cte_favorite_color_2│  │            │
//!     Relationship ──► │   │                 │  └───────────┬────────┘  │            │
//!   "favorite_color"   │ 61│favorite_color   │   {<json>}   │    01     │ peoples    │
//!                      ├───┼─────────────────┼──┬───────────┴───────────┼────────────┤
//!                      │   │                 │  │cte_articles_comments_3│            │
//!     Relationship ──► │   │                 │  └───────────┬───────────┤            │
//! "articles.comments"  │ 37│articles.comments│   {<json>}   │    42     │ articles   │
//!                      │ 30│articles.comments│   {<json>}   │    42     │ articles   │
//!                      │ 31│articles.comments│   {<json>}   │    43     │ articles   │
//!                      │ 32│articles.comments│   {<json>}   │    43     │ articles   │
//!                      │ 33│articles.comments│   {<json>}   │    44     │ articles   │
//!                      │ 35│articles.comments│   {<json>}   │    44     │ articles   │
//!                      └───┴─────────────────┴──────────────┴───────────┴────────────┘
//! ```
//!
//! The _Postgres_ response can be deserialized with [Ciboulette2PgRow](Ciboulette2PgRow).
//!
//! To then build the ciboulette response, one can convert the [Ciboulette2PgRow](Ciboulette2PgRow) to [Ciboulette response element](CibouletteResponseElement).

#![warn(clippy::all)]
mod builder;
mod ciboulette_row;
mod errors;
mod response_type;
mod safe_ident;
mod table_store;
#[cfg(test)]
mod tests;

mod value;

use arcstr::ArcStr;
pub use builder::Ciboulette2PgArguments;
pub use builder::Ciboulette2PgBuilder;
use ciboulette::*;
pub use ciboulette_row::Ciboulette2PgRow;
pub use errors::Ciboulette2PgError;
use getset::Getters;
use messy_json::{
    MessyJsonExpected, MessyJsonInner, MessyJsonNullType, MessyJsonObject, MessyJsonObjectValue,
    MessyJsonValue,
};
use response_type::Ciboulette2PgResponseType;
pub use safe_ident::Ciboulette2PgSafeIdent;
use safe_ident::Ciboulette2PgSafeIdentModifier;
use safe_ident::{
    CIBOULETTE_CTE_FINAL_MAIN_DATA, CIBOULETTE_DATA_IDENT, CIBOULETTE_DATA_SUFFIX,
    CIBOULETTE_EMPTY_IDENT, CIBOULETTE_ID_IDENT, CIBOULETTE_INSERT_SUFFIX,
    CIBOULETTE_MAIN_IDENTIFIER, CIBOULETTE_RELATED_ID_IDENT, CIBOULETTE_RELATED_TYPE_IDENT,
    CIBOULETTE_REL_PREFIX, CIBOULETTE_SORT_PREFIX, CIBOULETTE_TYPE_IDENT, CIBOULETTE_UPDATE_SUFFIX,
    TEXT_IDENT,
};
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::convert::TryFrom;
use std::sync::Arc;
pub use table_store::{Ciboulette2PgId, Ciboulette2PgTable, Ciboulette2PgTableStore};
pub use value::Ciboulette2PgValue;
const POSTGRES_QUOTE: &[u8] = b"\"";
