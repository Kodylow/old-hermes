use std::fs::read_to_string;

use axum::{extract::State, http::StatusCode, Json};
use axum_macros::debug_handler;
use tracing::{error, info};

use crate::{error::AppError, state::AppState, types::nostr::RegisterParams};

pub mod lnurlp;
pub mod nostr;

#[axum_macros::debug_handler]
pub async fn handle_readme() -> String {
    let readme = read_to_string("README.md").expect("Could not read README.md");
    info!("Serving README.md");
    readme
}

#[debug_handler]
pub async fn register(
    State(mut state): State<AppState>,
    Json(params): Json<RegisterParams>,
) -> Result<Json<bool>, AppError> {
    info!(
        "register called with nostr_pubkey: {:?}",
        params.nostr_pubkey
    );

    let name = params.name.clone().ok_or_else(|| AppError {
        error: anyhow::anyhow!("Name not provided"),
        status: StatusCode::BAD_REQUEST,
    })?;

    let mut nostr_json = state.nostr_json.clone();

    // if not registered, add to nostr.json
    if !nostr_json.names.contains_key(&name) {
        nostr_json.names.insert(name, params.nostr_pubkey.clone());

        // write nostr.json to disk
        let nostr_json_str = serde_json::to_string_pretty(&nostr_json)?;
        std::fs::write("nostr.json", nostr_json_str)?;

        // set to state
        state.nostr_json = nostr_json;
    } else {
        error!("Name already registered");
        return Err(AppError {
            error: anyhow::anyhow!("Name already registered"),
            status: StatusCode::BAD_REQUEST,
        });
    }

    Ok(Json(true))
}
