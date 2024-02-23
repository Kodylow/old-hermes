use anyhow::Result;
use config::CONFIG;
use multimint::MultiMint;
use nostr_sdk::Client;

use crate::config;
use crate::model::ModelManager;

#[derive(Clone)]
pub struct AppState {
    pub fm: MultiMint,
    pub mm: ModelManager,
    pub nostr: Client,
}

impl AppState {
    pub async fn new() -> Result<Self> {
        let fm = MultiMint::new(CONFIG.fm_db_path.clone()).await?;
        let mm = ModelManager::new().await?;
        let nostr = nostr_sdk::Client::new(&CONFIG.nostr_sk);
        nostr.add_relay(CONFIG.default_relay.as_str()).await?;
        nostr.connect().await;

        Ok(Self { fm, mm, nostr })
    }
}
