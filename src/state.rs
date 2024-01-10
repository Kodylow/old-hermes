use std::{collections::BTreeMap, path::PathBuf};

use fedimint_client::ClientArc;
use nostr_sdk::Client;

use crate::{config, model::ModelManager};

use anyhow::Result;
use config::CONFIG;
use fedimint_core::db::Database;
use fedimint_ln_client::LightningClientInit;
use fedimint_mint_client::MintClientInit;
use fedimint_wallet_client::WalletClientInit;

#[derive(Clone)]
pub struct AppState {
    pub fm_clients: Arc<Mutex<BTreeMap<FederationId, ClientArc>>>,
    pub mm: ModelManager,
    pub nostr: Client,
}

pub async fn load_existing_clients() -> Arc<Mutex<BTreeMap<FederationId, ClientArc>>> {
    let mut clients = BTreeMap::new();
    let fm_dbs = CONFIG
        .fm_db_path
        .read_dir()
        .expect("Failed to read fm db path")
        .flatten();
    for db in fm_dbs {
        let client_db_path = db.path();
        let client = load_fedimint_client(client_db_path)
            .await
            .expect("Failed to load client");
        let federation_id = client.federation_id();
        clients.insert(federation_id, client);
    }

    Arc::new(Mutex::new(clients))
}

pub async fn load_fedimint_client(file_path: PathBuf) -> Result<ClientArc> {
    let fedimint_client_path = CONFIG.fm_db_path.join(file_path);

    let db = Database::new(
        fedimint_rocksdb::RocksDb::open(CONFIG.fm_db_path.clone())?,
        Default::default(),
    );
    let mut client_builder = fedimint_client::Client::builder();

    client_builder.with_database(db);
    client_builder.with_module(WalletClientInit(None));
    client_builder.with_module(MintClientInit);
    client_builder.with_module(LightningClientInit);
    client_builder.with_primary_module(1);
    let client_secret = match client_builder.load_decodable_client_secret().await {
        Ok(secret) => secret,
        Err(_) => {
            let secret = PlainRootSecretStrategy::random(&mut thread_rng());
            store_encodable_client_secret(client_builder.db(), secret)
                .await
                .map_err(|e| anyhow!("Failed to store client secret: {}", e))?;
            secret
        }
    };
    let root_secret = PlainRootSecretStrategy::to_root_secret(&client_secret);
    let client_res = client_builder.build(root_secret).await?;

    Ok(client_res)
}

pub async fn store_encodable_client_secret<T: Encodable>(
    db: &Database,
    secret: T,
) -> anyhow::Result<()> {
    let mut dbtx = db.begin_transaction().await;

    // Don't overwrite an existing secret
    match dbtx.get_value(&EncodedClientSecretKey).await {
        Some(_) => bail!("Encoded client secret already exists, cannot overwrite"),
        None => {
            let encoded_secret = consensus_encode_to_vec(&secret);
            dbtx.insert_entry(&EncodedClientSecretKey, &encoded_secret)
                .await;
            dbtx.commit_tx().await;
            Ok(())
        }
    }
}

pub async fn load_nostr_client() -> Result<Client> {
    let client = nostr_sdk::Client::new(&CONFIG.nostr_sk);

    client.add_relay(CONFIG.default_relay.as_str()).await?;
    client.connect().await;

    Ok(client)
}
