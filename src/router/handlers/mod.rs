use std::fs::read_to_string;

use axum::{extract::State, http::StatusCode, Json};
use axum_macros::debug_handler;
use tracing::{error, info};

use crate::{
    error::AppError,
    model::nip05relays::{Nip05RelaysBmc, Nip05RelaysForCreate},
    state::AppState,
};

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
    Json(params): Json<Nip05RelaysForCreate>,
) -> Result<Json<bool>, AppError> {
    info!("register called with pubkey: {:?}", params.pubkey);
    match Nip05RelaysBmc::register(&mut state.mm, params).await {
        Ok(_) => Ok(Json(true)),
        Err(e) => {
            error!("Error registering: {:?}", e);
            return Err(AppError {
                error: anyhow::anyhow!("Error registering"),
                status: StatusCode::INTERNAL_SERVER_ERROR,
            });
        }
    }
}
