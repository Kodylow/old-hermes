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
    error::AppError, model::userrelays::UserRelaysBmc, router::handlers::NameOrPubkey,
    state::AppState,
};

use super::UserRelays;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserWellKnownParams {
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserWellKnown {
    pub names: HashMap<String, XOnlyPublicKey>,
    pub relays: HashMap<XOnlyPublicKey, Vec<String>>,
}

impl UserWellKnown {
    pub fn from_db(nip05relays: UserRelays) -> Self {
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
    Query(params): Query<UserWellKnownParams>,
    State(state): State<AppState>,
) -> Result<Json<UserWellKnown>, AppError> {
    info!("nip05_well_known called with name: {:?}", params.name);
    let nip05relays = UserRelaysBmc::get_by(&state.mm, NameOrPubkey::Name, &params.name).await?;

    let nip05_well_known = UserWellKnown::from_db(nip05relays);

    Ok(Json(nip05_well_known))
}
