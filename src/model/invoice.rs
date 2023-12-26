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
    pub id: i64,
    pub op_id: String,
    pub bolt11: String,
    pub settled: bool,
}

#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct InvoiceForCreate {
    pub op_id: String,
    pub bolt11: String,
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
    pub async fn create(mm: &ModelManager, inv_c: InvoiceForCreate) -> Result<i64> {
        base::create::<Self, _>(mm, inv_c).await
    }

    pub async fn get_by_op_id(mm: &ModelManager, op_id: &str) -> Result<Invoice> {
        info!("get_by_op_id called with op_id: {}", op_id);
        let inv: Invoice = sqlb::select()
            .table(Self::TABLE)
            .columns(Invoice::field_names())
            .and_where("op_id", "=", op_id)
            .fetch_optional(mm.db())
            .await?
            .ok_or(anyhow!("No invoice found with op_id: {}", op_id))?;
        Ok(inv)
    }

    pub async fn settle(mm: &ModelManager, id: i64) -> Result<()> {
        let inv_u = InvoiceForUpdate { settled: true };
        base::update::<Self, _>(mm, id, inv_u).await
    }

    pub async fn delete(mm: &ModelManager, id: i64) -> Result<()> {
        base::delete::<Self>(mm, id.into()).await
    }
}
