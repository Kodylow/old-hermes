use anyhow::anyhow;
use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::{
    config::CONFIG,
    error::AppError,
    model::nip05relays::{Nip05RelaysBmc, Nip05RelaysForCreate},
    state::AppState,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Nip05Params {
    pub pubkey: String,
    pub name: String,
    pub relays: Option<Vec<String>>,
}

#[axum_macros::debug_handler]
pub async fn handle_register(
    State(mut state): State<AppState>,
    Json(params): Json<Nip05Params>,
) -> Result<Json<bool>, AppError> {
    info!("register called with pubkey: {:?}", params.pubkey);

    let relays = params
        .relays
        .unwrap_or_else(|| vec![CONFIG.default_relay.clone()]);

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
