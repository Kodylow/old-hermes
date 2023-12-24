use super::{
    base::{self, DbBmc},
    ModelManager,
};
use anyhow::Result;
use serde::Serialize;
use sqlb::Fields;
use sqlx::FromRow;

#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct Relay {
    pub id: i64,
    pub relay: String,
}

#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct RelayForCreate {
    pub relay: String,
}

#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct RelayForUpdate {
    pub relay: Option<String>,
}

pub struct RelayBmc;

impl DbBmc for RelayBmc {
    const TABLE: &'static str = "relay";
}

impl RelayBmc {
    pub async fn create(mm: &ModelManager, relay_c: RelayForCreate) -> Result<i64> {
        base::create::<Self, _>(mm, relay_c).await
    }

    pub async fn get(mm: &ModelManager, id: i64) -> Result<Relay> {
        base::get::<Self, _>(mm, id).await
    }

    pub async fn get_all(mm: &ModelManager) -> Result<Vec<Relay>> {
        base::get_all::<Self, _>(mm).await
    }

    pub async fn list(mm: &ModelManager) -> Result<Vec<Relay>> {
        base::list::<Self, _>(mm).await
    }

    pub async fn update(mm: &ModelManager, id: i64, relay_u: RelayForUpdate) -> Result<()> {
        base::update::<Self, _>(mm, id, relay_u).await
    }

    pub async fn delete(mm: &ModelManager, id: i64) -> Result<()> {
        base::delete::<Self>(mm, id).await
    }
}
