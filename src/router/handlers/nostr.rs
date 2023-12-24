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

// let client = nostr_sdk::Client::new(&Keys::generate());
//     client.add_relay("wss://relay.damus.io", None).await?;
//     client
//         .add_relay("wss://nostr.mutinywallet.com", None)
//         .await?;
//     client.connect().await;

//     let filter = Filter::new()
//         .kind(Kind::Metadata)
//         .author(params.nostr_pubkey)
//         .limit(1);

//     let events = client.get_events_of(vec![filter], None).await?;

//     if let Some(event) = events.first() {
//         let metadata: Metadata = serde_json::from_str(&event.content)?;
//         println!("nip5: {:?}", metadata.nip05);
//     }

//     client
//         .send_direct_msg(params.nostr_pubkey, "connected!".to_string(), None)
//         .await?;
