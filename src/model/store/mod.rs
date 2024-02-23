use anyhow::{anyhow, Result};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

use crate::config::CONFIG;

pub type Db = Pool<Postgres>;

pub async fn new_db_pool() -> Result<Db> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&CONFIG.pg_db)
        .await
        .map_err(|ex| anyhow!("Could not connect to database: {}", ex))
}
