use anyhow::anyhow;
use axum::{extract::State, http::StatusCode, Json};
use serde::Deserialize;
use tracing::info;

use crate::{
    config::CONFIG,
    error::AppError,
    model::app_user_relays::{AppUserRelaysBmc, AppUserRelaysForCreate},
    state::AppState,
};

use crate::router::SupportedDmType;

#[derive(Debug, Clone, Deserialize)]
pub struct UserParams {
    pub pubkey: String,
    pub name: String,
    pub dm_type: SupportedDmType,
    pub relays: Option<Vec<String>>,
}

#[axum_macros::debug_handler]
pub async fn handle_register(
    State(mut state): State<AppState>,
    Json(params): Json<UserParams>,
) -> Result<Json<bool>, AppError> {
    info!("register called with pubkey: {:?}", params.pubkey);

    let relays = match params.dm_type {
        SupportedDmType::Nostr => params
            .relays
            .unwrap_or_else(|| vec![CONFIG.default_relay.clone()]),
        SupportedDmType::XMPP => {
            if params.relays.clone().is_some_and(|r| r.len() != 1) {
                return Err(AppError::new(
                    StatusCode::BAD_REQUEST,
                    anyhow!("XMPP requires exactly one chat server"),
                ));
            } else {
                params
                    .relays
                    .unwrap_or_else(|| vec![CONFIG.xmpp_chat_server.clone()])
            }
        }
    };

    let nip05relays_c = AppUserRelaysForCreate {
        pubkey: params.pubkey,
        name: params.name,
        dm_type: params.dm_type.to_string(),
        relays,
    };

    match AppUserRelaysBmc::register(&mut state.mm, nip05relays_c).await {
        Ok(_) => Ok(Json(true)),
        Err(e) => Err(AppError::new(
            StatusCode::BAD_REQUEST,
            anyhow!("Error registering nip05relays {:?}", e),
        )),
    }
}
