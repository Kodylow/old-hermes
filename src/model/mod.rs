mod base;
pub mod invoice;
pub mod relay;
mod store;
pub mod user;
pub mod userrelays;

use crate::model::store::{new_db_pool, Db};
use anyhow::Result;

#[derive(Clone, Debug)]
pub struct ModelManager {
    db: Db,
}

impl ModelManager {
    /// Constructor
    pub async fn new() -> Result<Self> {
        let db = new_db_pool().await?;

        Ok(ModelManager { db })
    }

    /// Returns the sqlx db pool reference.
    /// (Only for the model layer)
    pub(in crate::model) fn db(&self) -> &Db {
        &self.db
    }
}
