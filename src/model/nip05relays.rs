use crate::router::handlers::{nostr::Nip05Relays, NameOrPubkey};

use super::{
    base::{self, DbBmc},
    nip05::{Nip05, Nip05Bmc, Nip05ForCreate},
    relay::{RelayBmc, RelayForCreate},
    ModelManager,
};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlb::Fields;
use sqlb::HasFields;
use sqlx::FromRow;

#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct Nip05Relay {
    pub nip05_id: i64,
    pub relay_id: i64,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Nip05RelaysForCreate {
    pub pubkey: String,
    pub name: String,
    pub relays: Vec<String>,
}

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Nip05RelaysForUpdate {
    pub pubkey: Option<String>,
    pub name: Option<String>,
    pub relays: Option<Vec<String>>,
}

pub struct Nip05RelaysBmc;

impl DbBmc for Nip05RelaysBmc {
    const TABLE: &'static str = "nip05relays";
}

impl Nip05RelaysBmc {
    pub async fn register(mm: &ModelManager, nip05relays_c: Nip05RelaysForCreate) -> Result<()> {
        let tx = mm.db().begin().await?;
        let nip05_c = Nip05ForCreate {
            pubkey: nip05relays_c.pubkey,
            name: nip05relays_c.name,
        };
        let nip05_id = base::create::<Self, _>(&mm, nip05_c).await?;

        for relay in nip05relays_c.relays {
            let relay_c = RelayForCreate { relay };
            let relay_id = base::create::<Self, _>(&mm, relay_c).await?;
            let nip05relay = Nip05Relay {
                nip05_id: nip05_id,
                relay_id: relay_id,
            };
            base::create::<Self, _>(&mm, nip05relay).await?;
        }
        tx.commit().await?;

        Ok(())
    }

    pub async fn get_by(mm: &ModelManager, field: NameOrPubkey, val: &str) -> Result<Nip05Relays> {
        let db = mm.db();

        let nip05: Nip05 = Nip05Bmc::get_by(mm, field, val).await?;
        let nip05relay: Vec<Nip05Relay> =
            sqlb::select()
                .table(Self::TABLE)
                .columns(Nip05Relay::field_names())
                .and_where("nip05_id", "=", nip05.id)
                .fetch_all(db)
                .await?;

        let relay_ids: Vec<i64> = nip05relay
            .into_iter()
            .map(|nip05relay| nip05relay.relay_id)
            .collect();

        let relays = RelayBmc::get_many(mm, &relay_ids).await?;

        let nip05relays = Nip05Relays {
            pubkey: nip05.pubkey,
            name: nip05.name,
            relays: relays
                .into_iter()
                .map(|relay| relay.relay.to_string())
                .collect(),
        };

        Ok(nip05relays)
    }
}
