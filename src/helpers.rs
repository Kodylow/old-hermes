use std::collections::HashMap;

use crate::{
    error::AppError,
    models::nostr::{Nip05WellKnown, Nip05WellKnownParams},
};
use axum::http::StatusCode;

pub async fn get_pubkey_and_relays(
    nostr_json: &Nip05WellKnown,
    params: &Nip05WellKnownParams,
) -> Result<Nip05WellKnown, AppError> {
    let pubkey = nostr_json.names.get(&params.name).ok_or_else(|| AppError {
        error: anyhow::anyhow!("Username not found"),
        status: StatusCode::NOT_FOUND,
    })?;

    let relays = nostr_json.relays.get(pubkey).ok_or_else(|| AppError {
        error: anyhow::anyhow!("Relays not found"),
        status: StatusCode::NOT_FOUND,
    })?;

    let mut names = HashMap::new();
    names.insert(params.name.clone(), pubkey.clone());

    let mut relays_map = HashMap::new();
    relays_map.insert(pubkey.clone(), relays.clone());

    Ok(Nip05WellKnown {
        names,
        relays: relays_map,
    })
}
