use fedimint_client::ClientArc;

use crate::{config, models::nostr::Nip05WellKnown};

use anyhow::Result;
use config::CONFIG;
use fedimint_client::{get_config_from_db, FederationInfo};
use fedimint_core::db::Database;
use fedimint_ln_client::LightningClientInit;
use fedimint_mint_client::MintClientInit;
use fedimint_wallet_client::WalletClientInit;

pub async fn load_fedimint_client() -> Result<ClientArc> {
    let db = Database::new(
        fedimint_rocksdb::RocksDb::open(CONFIG.db_path.clone())?,
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

#[derive(Debug, Clone)]
pub struct AppState {
    pub fm_client: ClientArc,
    pub nostr_json: Nip05WellKnown,
}
