#![allow(dead_code)]

use anyhow::Result;
use nostr::EventId;
use serde::Serialize;
use sqlb::Fields;
use sqlx::FromRow;

use super::base::{self, DbBmc};
use super::ModelManager;

#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct Zap {
    pub id: i32,
    pub request: String,
    pub event_id: Option<String>,
}

#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct ZapForUpdate {
    pub event_id: String,
}

pub struct ZapBmc;

impl DbBmc for ZapBmc {
    const TABLE: &'static str = "zaps";
}

impl ZapBmc {
    pub async fn create(mm: &ModelManager, inv_c: Zap) -> Result<i32> {
        base::create::<Self, _>(mm, inv_c).await
    }

    pub async fn get(mm: &ModelManager, id: i32) -> Result<Zap> {
        base::get::<Self, _>(mm, id).await
    }

    pub async fn set_event_id(mm: &ModelManager, id: i32, event_id: EventId) -> Result<()> {
        let u = ZapForUpdate {
            event_id: event_id.to_hex(),
        };
        base::update::<Self, _>(mm, id, u).await?;
        Ok(())
    }

    pub async fn delete(mm: &ModelManager, id: i32) -> Result<()> {
        base::delete::<Self>(mm, id).await
    }
}
