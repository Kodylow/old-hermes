use super::{
    base::{self, DbBmc},
    ModelManager,
};
use anyhow::Result;
use serde::Serialize;
use sqlb::Fields;
use sqlx::FromRow;

#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct Nip05 {
    pub id: i64,
    pub pubkey: String,
    pub name: String,
}

#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct Nip05ForCreate {
    pub pubkey: String,
    pub name: String,
}

#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct Nip05ForUpdate {
    pub pubkey: Option<String>,
    pub name: Option<String>,
}

pub struct Nip05Bmc;

impl DbBmc for Nip05Bmc {
    const TABLE: &'static str = "nip05";
}

impl Nip05Bmc {
    pub async fn create(mm: &ModelManager, nip05_c: Nip05ForCreate) -> Result<i64> {
        base::create::<Self, _>(mm, nip05_c).await
    }

    pub async fn get(mm: &ModelManager, id: i64) -> Result<Nip05> {
        base::get::<Self, _>(mm, id).await
    }

    pub async fn list(mm: &ModelManager) -> Result<Vec<Nip05>> {
        base::list::<Self, _>(mm).await
    }

    pub async fn update(mm: &ModelManager, id: i64, nip05_u: Nip05ForUpdate) -> Result<()> {
        base::update::<Self, _>(mm, id, nip05_u).await
    }

    pub async fn delete(mm: &ModelManager, id: i64) -> Result<()> {
        base::delete::<Self>(mm, id).await
    }
}
