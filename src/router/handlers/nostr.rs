use axum::{
    extract::{Query, State},
    Json,
};
use tracing::info;

use crate::{
    error::AppError,
    models::nostr::{Nip05WellKnown, Nip05WellKnownParams},
    state::AppState,
    utils::get_pubkey_and_relays,
};

#[axum_macros::debug_handler]
pub async fn nip05_well_known(
    Query(params): Query<Nip05WellKnownParams>,
    State(state): State<AppState>,
) -> Result<Json<Nip05WellKnown>, AppError> {
    info!("nip05_well_known called with name: {:?}", params.name);

    let res = get_pubkey_and_relays(&state.nostr_json, &params).await?;

    Ok(Json(res))
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
