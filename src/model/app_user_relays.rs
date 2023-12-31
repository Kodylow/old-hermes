use crate::router::handlers::{nostr::AppUserRelays, NameOrPubkey};

use super::{
    app_user::{AppUser, AppUserBmc, AppUserForCreate},
    base::{self, DbBmc},
    relay::{RelayBmc, RelayForCreate},
    ModelManager,
};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlb::Fields;
use sqlb::HasFields;
use sqlx::FromRow;

#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct AppUserRelay {
    pub app_user_id: i32,
    pub relay_id: i32,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct AppUserRelaysForCreate {
    pub pubkey: String,
    pub name: String,
    pub dm_type: String,
    pub relays: Vec<String>,
}

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct AppUserRelaysForUpdate {
    pub pubkey: Option<String>,
    pub name: Option<String>,
    pub dm_type: Option<String>,
    pub relays: Option<Vec<String>>,
}

pub struct AppUserRelaysBmc;

impl DbBmc for AppUserRelaysBmc {
    const TABLE: &'static str = "app_user_relays";
}

impl AppUserRelaysBmc {
    pub async fn register(
        mm: &ModelManager,
        app_user_relays_c: AppUserRelaysForCreate,
    ) -> Result<()> {
        let tx = mm.db().begin().await?;
        let user_c = AppUserForCreate {
            pubkey: app_user_relays_c.pubkey,
            name: app_user_relays_c.name,
            dm_type: app_user_relays_c.dm_type,
        };
        let user_id = base::create::<Self, _>(mm, user_c).await?;

        for relay in app_user_relays_c.relays {
            let relay_c = RelayForCreate { relay };
            let relay_id = base::create::<Self, _>(mm, relay_c).await?;
            let userrelay = AppUserRelay {
                app_user_id: user_id,
                relay_id,
            };
            base::create::<Self, _>(mm, userrelay).await?;
        }
        tx.commit().await?;

        Ok(())
    }

    pub async fn get_by_id(mm: &ModelManager, id: i32) -> Result<AppUserRelays> {
        let db = mm.db();

        let user: AppUser = AppUserBmc::get(mm, id).await?;
        let userrelay: Vec<AppUserRelay> = sqlb::select()
            .table(Self::TABLE)
            .columns(AppUserRelay::field_names())
            .and_where("app_user_id", "=", user.id)
            .fetch_all(db)
            .await?;

        let relay_ids: Vec<i32> = userrelay
            .into_iter()
            .map(|userrelay| userrelay.relay_id)
            .collect();

        let relays = RelayBmc::get_many(mm, &relay_ids).await?;

        let userrelays = AppUserRelays {
            app_user_id: user.id,
            pubkey: user.pubkey,
            name: user.name,
            dm_type: user.dm_type,
            relays: relays
                .into_iter()
                .map(|relay| relay.relay.to_string())
                .collect(),
        };

        Ok(userrelays)
    }

    pub async fn get_by(
        mm: &ModelManager,
        field: NameOrPubkey,
        val: &str,
    ) -> Result<AppUserRelays> {
        let db = mm.db();

        let user: AppUser = AppUserBmc::get_by(mm, field, val).await?;
        let userrelay: Vec<AppUserRelay> = sqlb::select()
            .table(Self::TABLE)
            .columns(AppUserRelay::field_names())
            .and_where("app_user_id", "=", user.id)
            .fetch_all(db)
            .await?;

        let relay_ids: Vec<i32> = userrelay
            .into_iter()
            .map(|userrelay| userrelay.relay_id)
            .collect();

        let relays = RelayBmc::get_many(mm, &relay_ids).await?;

        let userrelays = AppUserRelays {
            app_user_id: user.id,
            pubkey: user.pubkey,
            name: user.name,
            dm_type: user.dm_type,
            relays: relays
                .into_iter()
                .map(|relay| relay.relay.to_string())
                .collect(),
        };

        Ok(userrelays)
    }
}
