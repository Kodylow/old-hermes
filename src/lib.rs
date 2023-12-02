use anyhow::Result;
use fedimint_client::{ClientArc, FederationInfo};
use fedimint_core::db::mem_impl::MemDatabase;
use fedimint_ln_client::LightningClientGen;
use fedimint_mint_client::MintClientGen;
use fedimint_wallet_client::WalletClientGen;

use crate::CONFIG;

pub mod config;
pub mod routes;

pub async fn load_fedimint_client() -> Result<ClientArc> {
    let federation_info = FederationInfo::from_invite_code(CONFIG.invite_code.clone()).await?;
    let mut client_builder = fedimint_client::Client::builder();
    client_builder.with_module(WalletClientGen(None));
    client_builder.with_module(MintClientGen);
    client_builder.with_module(LightningClientGen);
    client_builder.with_database(MemDatabase::new());
    client_builder.with_primary_module(1);
    client_builder.with_federation_info(federation_info);
    let client_res = client_builder.build(CONFIG.root_secret.clone()).await?;

    Ok(client_res)
}
