use std::fs::read_to_string;

use anyhow::Result;
use axum::{
    routing::{get, post},
    Router,
};
use config::CONFIG;
use fedimint_client::{get_config_from_db, ClientArc, FederationInfo};
use fedimint_core::db::Database;
use fedimint_ln_client::LightningClientInit;
use fedimint_mint_client::MintClientInit;
use fedimint_wallet_client::WalletClientInit;
use models::nostr::Nip05WellKnown;
use routes::nip05_well_known;
use state::AppState;

use crate::routes::{handle_readme, lnurlp_callback, lnurlp_verify, lnurlp_well_known, register};

pub mod config;
pub mod error;
pub mod helpers;
pub mod models;
pub mod routes;
pub mod state;
pub mod utils;

pub async fn create_app() -> Result<Router> {
    let state = AppState {
        fm_client: load_fedimint_client().await?,
        nostr_json: get_nostr_json(),
    };

    let app = Router::new()
        .route("/", get(handle_readme))
        .route("/health", get(|| async { "OK" }))
        .route("/register", post(register))
        .route("/.well-known/nostr.json", get(nip05_well_known))
        .route("/.well-known/lnurlp/:username", get(lnurlp_well_known))
        .route("/lnurlp/:username/callback", get(lnurlp_callback))
        .route("/lnurlp/:username/verify/:operation_id", get(lnurlp_verify))
        .with_state(state);

    Ok(app)
}

async fn load_fedimint_client() -> Result<ClientArc> {
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

fn get_nostr_json() -> Nip05WellKnown {
    let nostr_str = read_to_string("nostr.json").expect("Could not read nostr.json");
    serde_json::from_str::<Nip05WellKnown>(&nostr_str).expect("Invalid nostr.json")
}
