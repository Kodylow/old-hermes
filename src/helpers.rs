use crate::{error::AppError, models::nostr::Nip05WellKnownParams, state::AppState};
use axum::http::StatusCode;
use nostr::prelude::XOnlyPublicKey;

pub async fn get_pubkey_and_relays(
    state: &AppState,
    params: &Nip05WellKnownParams,
) -> Result<(XOnlyPublicKey, Vec<String>), AppError> {
    let pubkey = state
        .nostr_json
        .names
        .get(&params.name)
        .ok_or_else(|| AppError {
            error: anyhow::anyhow!("Username not found"),
            status: StatusCode::NOT_FOUND,
        })?;

    let relays = state
        .nostr_json
        .relays
        .get(pubkey)
        .ok_or_else(|| AppError {
            error: anyhow::anyhow!("Relays not found"),
            status: StatusCode::NOT_FOUND,
        })?;

    Ok((pubkey.clone(), relays.clone()))
}
