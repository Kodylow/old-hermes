#![allow(dead_code)]
use super::{
    base::{self, DbBmc},
    ModelManager,
};
use anyhow::{anyhow, Result};
use serde::Serialize;
use sqlb::{Fields, HasFields};
use sqlx::FromRow;
use tracing::info;

#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct Invoice {
    pub id: i32,
    pub op_id: String,
    pub bolt11: String,
    pub amount: i64,
    pub settled: bool,
}

#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct InvoiceForCreate {
    pub op_id: String,
    pub bolt11: String,
    pub amount: i64,
}

#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct InvoiceForUpdate {
    pub settled: bool,
}

pub struct InvoiceBmc;

impl DbBmc for InvoiceBmc {
    const TABLE: &'static str = "invoice";
}

impl InvoiceBmc {
    pub async fn create(mm: &ModelManager, inv_c: InvoiceForCreate) -> Result<i32> {
        base::create::<Self, _>(mm, inv_c).await
    }

    pub async fn get(mm: &ModelManager, id: i32) -> Result<Invoice> {
        base::get::<Self, _>(mm, id).await
    }

    pub async fn get_by_op_id(mm: &ModelManager, op_id: &str) -> Result<Invoice> {
        let inv: Invoice = sqlb::select()
            .table(Self::TABLE)
            .columns(Invoice::field_names())
            .and_where("op_id", "=", op_id)
            .fetch_optional(mm.db())
            .await?
            .ok_or(anyhow!("No invoice found with op_id: {}", op_id))?;
        Ok(inv)
    }

    pub async fn settle(mm: &ModelManager, id: i32) -> Result<Invoice> {
        let inv_u = InvoiceForUpdate { settled: true };
        base::update::<Self, _>(mm, id, inv_u).await?;
        Self::get(mm, id).await
    }

    pub async fn delete(mm: &ModelManager, id: i32) -> Result<()> {
        base::delete::<Self>(mm, id).await
    }
}
