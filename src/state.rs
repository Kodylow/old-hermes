use fedimint_client::ClientArc;
use nostr::key::FromSkStr;
use nostr::Keys;
use nostr_sdk::Client;

use crate::{config, model::ModelManager};

use anyhow::Result;
use config::CONFIG;
use fedimint_client::{get_config_from_db, FederationInfo};
use fedimint_core::db::Database;
use fedimint_ln_client::LightningClientInit;
use fedimint_mint_client::MintClientInit;
use fedimint_wallet_client::WalletClientInit;

#[derive(Clone)]
pub struct AppState {
    pub fm: ClientArc,
    pub mm: ModelManager,
    pub nostr: Client,
}

pub async fn load_fedimint_client() -> Result<ClientArc> {
    let db =
        Database::new(
            fedimint_rocksdb::RocksDb::open(CONFIG.fm_db_path.clone())?,
            Default::default(),
        );
    let mut client_builder = fedimint_client::Client::builder();
    if get_config_from_db(&db).await.is_none() {
        let federation_info = FederationInfo::from_invite_code(CONFIG.invite_code.clone()).await?;
        client_builder.with_federation_info(federation_info);
    };
    client_builder.with_database(db);
    client_builder.with_module(WalletClientInit(None));
    client_builder.with_module(MintClientInit);
    client_builder.with_module(LightningClientInit);
    client_builder.with_primary_module(1);
    let client_res = client_builder.build(CONFIG.root_secret.clone()).await?;

    Ok(client_res)
}

pub async fn load_nostr_client() -> Result<Client> {
    let keys = Keys::from_sk_str(&CONFIG.nostr_sk)?;
    let client = nostr_sdk::Client::new(&keys);

    Ok(client)
}
