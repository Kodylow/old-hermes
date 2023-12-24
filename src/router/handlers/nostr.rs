use axum::{
    extract::{Query, State},
    Json,
};
use tracing::info;

use crate::{
    error::AppError,
    state::AppState,
    types::nostr::{Nip05WellKnown, Nip05WellKnownParams},
};

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
