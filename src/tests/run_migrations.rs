use super::*;
use lazy_static::lazy_static;
use std::str::FromStr;
use uuid::Uuid;

mod embedded_migrations {
    use refinery::embed_migrations;
    embed_migrations!("./tests_migrations");
}

lazy_static! {
    static ref BASILIQ_DATABASE_URL: String =
        std::env::var("BASILIQ_DATABASE_URL").expect("the database url to be set");
    static ref BASILIQ_MANAGEMENT_POOL: sqlx::PgPool = {
        let num = num_cpus::get();
        sqlx::pool::PoolOptions::new()
            .min_connections(1)
            .max_connections(num as u32)
            .connect_lazy(&BASILIQ_DATABASE_URL)
            .expect("to initialize the management Postgres connection pool")
    };
}

pub async fn run_migrations(db_name: &str) {
    let mut config = refinery::config::Config::from_env_var("BASILIQ_DATABASE_URL")
        .expect("to parse the basiliq database url")
        .set_db_name(db_name);
    embedded_migrations::migrations::runner()
        .run_async(&mut config)
        .await
        .expect("to apply migrations");
}

pub async fn init_db() -> (String, sqlx::PgPool) {
    let db_name = format!("basiliq_test_{}", Uuid::new_v4());
    println!("Gen name {}", db_name);
    sqlx::query(format!("CREATE DATABASE \"{}\";", db_name.as_str()).as_str())
        .execute(&*BASILIQ_MANAGEMENT_POOL)
        .await
        .expect("to create a new database");
    println!("Created database");

    run_migrations(db_name.as_str()).await;
    println!("Running migrations");

    let conn_opt = sqlx::postgres::PgConnectOptions::from_str(&BASILIQ_DATABASE_URL)
        .expect("to parse the basiliq database url")
        .database(db_name.as_str());
    println!("Connecting");
    let pool = sqlx::pool::PoolOptions::new()
        .min_connections(1)
        .max_connections(3)
        .connect_with(conn_opt)
        .await
        .expect("to initialize the management Postgres connection pool");
    (db_name, pool)
}
