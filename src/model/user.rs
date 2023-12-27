#![allow(dead_code)]
use crate::router::handlers::NameOrPubkey;

use super::{
    base::{self, DbBmc},
    ModelManager,
};
use anyhow::{anyhow, Result};
use serde::Serialize;
use sqlb::Fields;
use sqlb::HasFields;
use sqlx::FromRow;

#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct User {
    pub id: i32,
    pub pubkey: String,
    pub name: String,
    pub dm_type: String,
}

#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct UserForCreate {
    pub pubkey: String,
    pub name: String,
    pub dm_type: String,
}

#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct UserForUpdate {
    pub pubkey: Option<String>,
    pub name: Option<String>,
    pub dm_type: Option<String>,
}

pub struct UserBmc;

impl DbBmc for UserBmc {
    const TABLE: &'static str = "user";
}

impl UserBmc {
    pub async fn create(mm: &ModelManager, user_c: UserForCreate) -> Result<i32> {
        base::create::<Self, _>(mm, user_c).await
    }

    pub async fn get(mm: &ModelManager, id: i32) -> Result<User> {
        base::get::<Self, _>(mm, id).await
    }

    pub async fn get_by(mm: &ModelManager, col: NameOrPubkey, val: &str) -> Result<User> {
        let column_name = match col {
            NameOrPubkey::Name => "name",
            NameOrPubkey::Pubkey => "pubkey",
        };

        let user: User = sqlb::select()
            .table(Self::TABLE)
            .columns(User::field_names())
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

    pub async fn list(mm: &ModelManager) -> Result<Vec<User>> {
        base::list::<Self, _>(mm).await
    }

    pub async fn update(mm: &ModelManager, id: i32, user_u: UserForUpdate) -> Result<()> {
        base::update::<Self, _>(mm, id, user_u).await
    }

    pub async fn delete(mm: &ModelManager, id: i32) -> Result<()> {
        base::delete::<Self>(mm, id).await
    }
}
