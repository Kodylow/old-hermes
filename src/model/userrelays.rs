use crate::router::handlers::{nostr::UserRelays, NameOrPubkey};

use super::{
    base::{self, DbBmc},
    relay::{RelayBmc, RelayForCreate},
    user::{User, UserBmc, UserForCreate},
    ModelManager,
};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlb::Fields;
use sqlb::HasFields;
use sqlx::FromRow;

#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct UserRelay {
    pub user_id: i32,
    pub relay_id: i32,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct UserRelaysForCreate {
    pub pubkey: String,
    pub name: String,
    pub dm_type: String,
    pub relays: Vec<String>,
}

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct UserRelaysForUpdate {
    pub pubkey: Option<String>,
    pub name: Option<String>,
    pub dm_type: Option<String>,
    pub relays: Option<Vec<String>>,
}

pub struct UserRelaysBmc;

impl DbBmc for UserRelaysBmc {
    const TABLE: &'static str = "userrelays";
}

impl UserRelaysBmc {
    pub async fn register(mm: &ModelManager, userrelays_c: UserRelaysForCreate) -> Result<()> {
        let tx = mm.db().begin().await?;
        let user_c = UserForCreate {
            pubkey: userrelays_c.pubkey,
            name: userrelays_c.name,
            dm_type: userrelays_c.dm_type,
        };
        let user_id = base::create::<Self, _>(&mm, user_c).await?;

        for relay in userrelays_c.relays {
            let relay_c = RelayForCreate { relay };
            let relay_id = base::create::<Self, _>(&mm, relay_c).await?;
            let userrelay = UserRelay {
                user_id: user_id,
                relay_id: relay_id,
            };
            base::create::<Self, _>(&mm, userrelay).await?;
        }
        tx.commit().await?;

        Ok(())
    }

    pub async fn get_by(mm: &ModelManager, field: NameOrPubkey, val: &str) -> Result<UserRelays> {
        let db = mm.db();

        let user: User = UserBmc::get_by(mm, field, val).await?;
        let userrelay: Vec<UserRelay> =
            sqlb::select()
                .table(Self::TABLE)
                .columns(UserRelay::field_names())
                .and_where("user_id", "=", user.id)
                .fetch_all(db)
                .await?;

        let relay_ids: Vec<i32> = userrelay
            .into_iter()
            .map(|userrelay| userrelay.relay_id)
            .collect();

        let relays = RelayBmc::get_many(mm, &relay_ids).await?;

        let userrelays = UserRelays {
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
