use std::fs::read_to_string;

use async_utility::futures_util::StreamExt;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use axum_macros::debug_handler;
use fedimint_client::oplog::UpdateStreamOrOutcome;
use fedimint_core::{core::OperationId, Amount};
use fedimint_ln_client::LightningClientModule;
use std::str::FromStr;
use tokio::sync::oneshot;
use tracing::{error, info};

use crate::{
    config::CONFIG,
    error::AppError,
    helpers::get_pubkey_and_relays,
    models::lnurl::{
        LnurlCallbackParams, LnurlCallbackResponse, LnurlStatus, LnurlType, LnurlWellKnownResponse,
    },
    models::nostr::{Nip05WellKnown, Nip05WellKnownParams, RegisterParams},
    AppState,
};

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

#[axum_macros::debug_handler]
pub async fn nip05_well_known(
    Query(params): Query<Nip05WellKnownParams>,
    State(state): State<AppState>,
) -> Result<Json<Nip05WellKnown>, AppError> {
    info!("nip05_well_known called with name: {:?}", params.name);

    let res = get_pubkey_and_relays(&state.nostr_json, &params).await?;

    Ok(Json(res))
}

#[axum_macros::debug_handler]
pub async fn lnurlp_well_known(
    Path(username): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<LnurlWellKnownResponse>, AppError> {
    // see if username exists in nostr.json
    info!("lnurlp_well_known called with username: {}", username);
    let nostr_json = &state.nostr_json;
    let pubkey = nostr_json.names.get(&username).ok_or_else(|| AppError {
        error: anyhow::anyhow!("Username not found"),
        status: StatusCode::NOT_FOUND,
    })?;

    let res = LnurlWellKnownResponse {
        callback: format!(
            "http://{}/lnurlp/{}/callback",
            CONFIG.domain,
            username.to_string()
        )
        .parse()?,
        max_sendable: Amount { msats: 100000 },
        min_sendable: Amount { msats: 1000 },
        metadata: "test metadata".to_string(),
        comment_allowed: None,
        tag: LnurlType::PayRequest,
        status: LnurlStatus::Ok,
        nostr_pubkey: Some(pubkey.clone()),
        allows_nostr: true,
    };

    Ok(Json(res))
}

#[axum_macros::debug_handler]
pub async fn lnurlp_callback(
    Path(username): Path<String>,
    Query(params): Query<LnurlCallbackParams>,
    State(state): State<AppState>,
) -> Result<Json<LnurlCallbackResponse>, AppError> {
    info!("lnurlp_callback called with username: {}", username);
    if username != "kody".to_string() {
        return Err(AppError {
            error: anyhow::anyhow!("Username not found"),
            status: StatusCode::NOT_FOUND,
        });
    }

    if params.amount < 1000 {
        return Err(AppError {
            error: anyhow::anyhow!("Amount too low"),
            status: StatusCode::BAD_REQUEST,
        });
    }

    let (op_id, pr) =
        state
            .fm_client
            .get_first_module::<LightningClientModule>()
            .create_bolt11_invoice(
                Amount {
                    msats: params.amount,
                },
                "test invoice".to_string(),
                None,
                (),
            )
            .await?;

    let verify_url = format!(
        "http://localhost:3000/lnurlp/{}/verify/{}",
        username,
        op_id.to_string()
    );

    let res = LnurlCallbackResponse {
        pr: pr.to_string(),
        success_action: None,
        status: LnurlStatus::Ok,
        reason: None,
        verify: verify_url.parse()?,
        routes: None,
    };

    Ok(Json(res))
}

#[axum_macros::debug_handler]
pub async fn lnurlp_verify(
    Path(params): Path<(String, OperationId)>,
    State(state): State<AppState>,
) -> Result<Json<bool>, AppError> {
    let (_username, operation_id) = params;
    let subscription = state
        .fm_client
        .get_first_module::<LightningClientModule>()
        .subscribe_ln_receive(operation_id)
        .await
        .expect("subscribing to a just created operation can't fail");

    match subscription {
        UpdateStreamOrOutcome::UpdateStream(stream) => {
            let (tx, rx) = oneshot::channel();
            tokio::spawn(async move {
                let mut stream = stream.fuse();
                tokio::select! {
                    _ = stream.next() => {
                        tx.send(true).unwrap();
                    }
                    _ = tokio::time::sleep(std::time::Duration::from_secs(10)) => {
                        tx.send(false).unwrap();
                    }
                }
            });
            info!("waiting for stream to complete");
            let res = rx.await.unwrap();
            return Ok(Json(res));
        }
        UpdateStreamOrOutcome::Outcome(outcome) => {
            return Err(AppError {
                error: anyhow::anyhow!("Operation status: {:?}", outcome),
                status: StatusCode::BAD_REQUEST,
            });
        }
    }
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
