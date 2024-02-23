#![allow(dead_code)]
use anyhow::{anyhow, Result};
use serde::Serialize;
use sqlb::{Fields, HasFields};
use sqlx::FromRow;

use super::base::{self, DbBmc};
use super::ModelManager;
use crate::router::handlers::NameOrPubkey;

#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct AppUser {
    pub id: i32,
    pub pubkey: String,
    pub name: String,
    pub dm_type: String,
    pub federation_id: String,
}

#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct AppUserForCreate {
    pub pubkey: String,
    pub name: String,
    pub dm_type: String,
    pub federation_id: String,
}

#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct AppUserForUpdate {
    pub pubkey: Option<String>,
    pub name: Option<String>,
    pub dm_type: Option<String>,
    pub federation_id: Option<String>,
}

pub struct AppUserBmc;

impl DbBmc for AppUserBmc {
    const TABLE: &'static str = "app_user";
}

impl AppUserBmc {
    pub async fn create(mm: &ModelManager, user_c: AppUserForCreate) -> Result<i32> {
        base::create::<Self, _>(mm, user_c).await
    }

    pub async fn get(mm: &ModelManager, id: i32) -> Result<AppUser> {
        base::get::<Self, _>(mm, id).await
    }

    pub async fn get_by(mm: &ModelManager, col: NameOrPubkey, val: &str) -> Result<AppUser> {
        let column_name = match col {
            NameOrPubkey::Name => "name",
            NameOrPubkey::Pubkey => "pubkey",
        };

        let user: AppUser = sqlb::select()
            .table(Self::TABLE)
            .columns(AppUser::field_names())
            .and_where(column_name, "=", val)
            .fetch_optional(mm.db())
            .await?
            .ok_or(anyhow!(
                "User not found in table '{}', {}: {}",
                Self::TABLE,
                column_name,
                val
            ))?;

        Ok(user)
    }

    pub async fn list(mm: &ModelManager) -> Result<Vec<AppUser>> {
        base::list::<Self, _>(mm).await
    }

    pub async fn update(mm: &ModelManager, id: i32, user_u: AppUserForUpdate) -> Result<()> {
        base::update::<Self, _>(mm, id, user_u).await
    }

    pub async fn delete(mm: &ModelManager, id: i32) -> Result<()> {
        base::delete::<Self>(mm, id).await
    }
}
