use anyhow::anyhow;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use fedimint_core::config::FederationId;
use serde::Deserialize;
use tracing::info;

use crate::config::CONFIG;
use crate::error::AppError;
use crate::model::app_user_relays::{AppUserRelaysBmc, AppUserRelaysForCreate};
use crate::router::SupportedDmType;
use crate::state::AppState;

#[derive(Debug, Clone, Deserialize)]
pub struct UserParams {
    pub pubkey: String,
    pub name: String,
    pub dm_type: SupportedDmType,
    pub federation_id: FederationId,
    pub relays: Option<Vec<String>>,
}

#[axum_macros::debug_handler]
pub async fn handle_register(
    State(state): State<AppState>,
    Json(params): Json<UserParams>,
) -> Result<Json<bool>, AppError> {
    info!("register called with pubkey: {:?}", params.pubkey);

    // Check if the federationId is in the multimint map
    if !state
        .fm
        .clients
        .lock()
        .await
        .contains_key(&params.federation_id)
    {
        return Err(AppError::new(
            StatusCode::BAD_REQUEST,
            anyhow!("FederationId not found in multimint map"),
        ));
    }

    let relays = match params.dm_type {
        SupportedDmType::Nostr => params
            .relays
            .unwrap_or_else(|| vec![CONFIG.default_relay.clone()]),
        SupportedDmType::Xmpp => {
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
        federation_id: params.federation_id.to_string(),
        name: params.name,
        dm_type: params.dm_type.to_string(),
        relays,
    };

    match AppUserRelaysBmc::register(&state.mm, nip05relays_c).await {
        Ok(_) => Ok(Json(true)),
        Err(e) => Err(AppError::new(
            StatusCode::BAD_REQUEST,
            anyhow!("Error registering nip05relays {:?}", e),
        )),
    }
}
