use async_utility::futures_util::StreamExt;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use fedimint_core::task::spawn;
use fedimint_core::{core::OperationId, Amount};
use fedimint_ln_client::{LightningClientModule, LnReceiveState};
use nostr::secp256k1::XOnlyPublicKey;
use std::str::FromStr;
use tracing::{error, info};

use crate::{
    config::CONFIG,
    error::AppError,
    model::{invoice::InvoiceBmc, nip05::Nip05Bmc},
    state::AppState,
    types::lnurl::{
        LnurlCallbackParams, LnurlCallbackResponse, LnurlStatus, LnurlType, LnurlVerifyResponse,
        LnurlWellKnownResponse,
    },
    types::NameOrPubkey,
};

#[axum_macros::debug_handler]
pub async fn well_known(
    Path(username): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<LnurlWellKnownResponse>, AppError> {
    // see if username exists in nostr.json
    info!("well_known called with username: {}", username);
    let nip05 = Nip05Bmc::get_by(&state.mm, NameOrPubkey::Name, &username).await?;

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
        nostr_pubkey: Some(XOnlyPublicKey::from_str(&nip05.pubkey)?),
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

    let ln = state.fm.get_first_module::<LightningClientModule>();

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
        "http://{}:{}/lnurlp/{}/verify/{}",
        CONFIG.domain,
        CONFIG.port,
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
    Path((username, op_id)): Path<(String, OperationId)>,
    State(state): State<AppState>,
) -> Result<Json<LnurlVerifyResponse>, AppError> {
    info!(
        "verify called with username: {}, op_id: {}",
        username, op_id
    );

    // Convert the operation id to an integer for the database lookup
    let invoice_id = op_id.to_string().parse::<i64>()?;

    // Use the operation id to look up the invoice
    let invoice = InvoiceBmc::get(&state.mm, invoice_id).await?;

    let verify_response = LnurlVerifyResponse {
        status: LnurlStatus::Ok,
        settled: invoice.settled,
        preimage: "".to_string(), // TODO: figure out how to get the preimage from fedimint client
        pr: invoice.bolt11,
    };

    Ok(Json(verify_response))
}
