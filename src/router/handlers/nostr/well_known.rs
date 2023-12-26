use std::collections::HashMap;

use axum::{
    extract::{Query, State},
    Json,
};
use nostr::prelude::XOnlyPublicKey;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use tracing::info;

use crate::{
    error::AppError, model::nip05relays::Nip05RelaysBmc, router::handlers::NameOrPubkey,
    state::AppState,
};

use super::Nip05Relays;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Nip05WellKnownParams {
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Nip05WellKnown {
    pub names: HashMap<String, XOnlyPublicKey>,
    pub relays: HashMap<XOnlyPublicKey, Vec<String>>,
}

impl Nip05WellKnown {
    pub fn from_db(nip05relays: Nip05Relays) -> Self {
        let mut names = HashMap::new();
        names.insert(
            nip05relays.name,
            XOnlyPublicKey::from_str(&nip05relays.pubkey).unwrap(),
        );
        let mut relays = HashMap::new();
        relays.insert(
            XOnlyPublicKey::from_str(&nip05relays.pubkey).unwrap(),
            nip05relays.relays,
        );
        Self { names, relays }
    }
}

#[axum_macros::debug_handler]
pub async fn handle_nip05_well_known(
    Query(params): Query<Nip05WellKnownParams>,
    State(state): State<AppState>,
) -> Result<Json<Nip05WellKnown>, AppError> {
    info!("nip05_well_known called with name: {:?}", params.name);
    let nip05relays = Nip05RelaysBmc::get_by(&state.mm, NameOrPubkey::Name, &params.name).await?;

    let nip05_well_known = Nip05WellKnown::from_db(nip05relays);

    Ok(Json(nip05_well_known))
}
