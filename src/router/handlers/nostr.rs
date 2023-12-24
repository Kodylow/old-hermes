use crate::{
    error::AppError,
    model::nip05relays::Nip05RelaysForCreate,
    state::AppState,
    types::nostr::{Nip05Params, Nip05WellKnown, Nip05WellKnownParams},
};
use anyhow::anyhow;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use axum_macros::debug_handler;
use tracing::info;

use crate::types::NameOrPubkey;

use crate::model::nip05relays::Nip05RelaysBmc;

#[axum_macros::debug_handler]
pub async fn nip05_well_known(
    Query(params): Query<Nip05WellKnownParams>,
    State(state): State<AppState>,
) -> Result<Json<Nip05WellKnown>, AppError> {
    info!("nip05_well_known called with name: {:?}", params.name);
    let nip05relays = Nip05RelaysBmc::get_by(&state.mm, NameOrPubkey::Name, &params.name).await?;

    let nip05_well_known = Nip05WellKnown::from_db(nip05relays);

    Ok(Json(nip05_well_known))
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
