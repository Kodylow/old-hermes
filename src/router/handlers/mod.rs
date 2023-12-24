use std::fs::read_to_string;

use crate::{
    error::AppError,
    model::nip05relays::{Nip05RelaysBmc, Nip05RelaysForCreate},
    state::AppState,
    types::nostr::Nip05Params,
};
use anyhow::anyhow;
use axum::{extract::State, http::StatusCode, Json};
use axum_macros::debug_handler;
use tracing::info;

pub mod lnurlp;
pub mod nostr;

#[axum_macros::debug_handler]
pub async fn handle_readme() -> String {
    let readme = read_to_string("README.md").expect("Could not read README.md");
    info!("Serving README.md");
    readme
}

#[debug_handler]
pub async fn register(
    State(mut state): State<AppState>,
    Json(params): Json<Nip05Params>,
) -> Result<Json<bool>, AppError> {
    info!("register called with pubkey: {:?}", params.pubkey);

    let relays = match params.relays {
        Some(relays) => relays,
        None => vec!["wss://nostr.mutinywallet.com".to_string()],
    };

    let nip05relays_c = Nip05RelaysForCreate {
        pubkey: params.pubkey,
        name: params.name,
        relays,
    };

    match Nip05RelaysBmc::register(&mut state.mm, nip05relays_c).await {
        Ok(_) => Ok(Json(true)),
        Err(e) => Err(AppError::new(
            StatusCode::BAD_REQUEST,
            anyhow!("Error registering nip05relays {:?}", e),
        )),
    }
}
