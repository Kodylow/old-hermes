use async_utility::futures_util::StreamExt;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use fedimint_core::task::spawn;
use fedimint_core::{core::OperationId, Amount};
use fedimint_ln_client::{LightningClientModule, LnReceiveState};
use tracing::{error, info};

use crate::{
    config::CONFIG,
    error::AppError,
    models::lnurl::{
        LnurlCallbackParams, LnurlCallbackResponse, LnurlStatus, LnurlType, LnurlWellKnownResponse,
    },
    router::state::AppState,
};

#[axum_macros::debug_handler]
pub async fn well_known(
    Path(username): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<LnurlWellKnownResponse>, AppError> {
    // see if username exists in nostr.json
    info!("well_known called with username: {}", username);
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
pub async fn callback(
    Path(username): Path<String>,
    Query(params): Query<LnurlCallbackParams>,
    State(state): State<AppState>,
) -> Result<Json<LnurlCallbackResponse>, AppError> {
    info!("callback called with username: {}", username);
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

    let ln = state.fm_client.get_first_module::<LightningClientModule>();

    let (op_id, pr) = ln
        .create_bolt11_invoice(
            Amount {
                msats: params.amount,
            },
            "test invoice".to_string(),
            None,
            (),
        )
        .await?;

    // create subscription to operation
    let subscription = ln
        .subscribe_ln_receive(op_id)
        .await
        .expect("subscribing to a just created operation can't fail");

    // spawn a task to wait for the operation to be updated
    spawn("waiting for invoice being paid", async move {
        let mut stream = subscription.into_stream();
        while let Some(op_state) = stream.next().await {
            match op_state {
                LnReceiveState::Created => {
                    info!("Invoice created, waiting for payment");
                }
                LnReceiveState::WaitingForPayment { invoice, timeout } => {
                    info!(
                        "Waiting for payment for invoice: {}, timeout: {:?}",
                        invoice, timeout
                    );
                }
                LnReceiveState::Canceled { reason } => {
                    error!("Payment canceled, reason: {:?}", reason);
                    break;
                }
                LnReceiveState::Funded => {
                    info!("Payment received, waiting for funds");
                }
                LnReceiveState::AwaitingFunds => {
                    info!("Awaiting funds");
                }
                LnReceiveState::Claimed => {
                    info!("Payment claimed");
                    break;
                }
            }
        }
    });

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
pub async fn verify(
    Path(_params): Path<(String, OperationId)>,
    State(_state): State<AppState>,
) -> Result<Json<bool>, AppError> {
    todo!();
}
