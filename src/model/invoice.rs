#![allow(dead_code)]
use super::{
    base::{self, DbBmc},
    ModelManager,
};
use anyhow::Result;
use serde::Serialize;
use sqlb::Fields;
use sqlx::FromRow;

#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct Invoice {
    pub id: i32,
    pub bolt11: String,
    pub settled: bool,
}

#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct InvoiceForCreate {
    pub id: i32,
    pub bolt11: String,
}

#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct InvoiceForUpdate {
    pub id: i32,
    pub settled: bool,
}

pub struct InvoiceBmc;

impl DbBmc for InvoiceBmc {
    const TABLE: &'static str = "invoice";
}

impl InvoiceBmc {
    pub async fn create(mm: &ModelManager, inv_c: InvoiceForCreate) -> Result<i64> {
        base::create::<Self, _>(mm, inv_c).await
    }

    pub async fn get(mm: &ModelManager, id: i32) -> Result<Invoice> {
        base::get::<Self, _>(mm, id.into()).await
    }

    pub async fn update(mm: &ModelManager, inv_u: InvoiceForUpdate) -> Result<()> {
        base::update::<Self, _>(mm, inv_u.id.into(), inv_u).await
    }

    pub async fn delete(mm: &ModelManager, id: i32) -> Result<()> {
        base::delete::<Self>(mm, id.into()).await
    }
}
