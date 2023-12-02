use std::fs::read_to_string;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use fedimint_core::Amount;
use fedimint_ln_client::LightningClientModule;
use tracing::info;

use crate::{
    error::AppError,
    models::{LnurlCallbackParams, LnurlCallbackResponse, LnurlStatus, LnurlWellKnownResponse},
    AppState,
};

#[axum_macros::debug_handler]
pub async fn handle_readme() -> String {
    let readme = read_to_string("README.md").expect("Could not read README.md");
    info!("Serving README.md");
    readme
}

pub async fn register() -> String {
    "register stub".to_string()
}

#[axum_macros::debug_handler]
pub async fn well_known(
    Path(username): Path<String>,
) -> Result<Json<LnurlWellKnownResponse>, AppError> {
    if username != "kody".to_string() {
        return Err(AppError {
            error: anyhow::anyhow!("Username not found"),
            status: StatusCode::NOT_FOUND,
        });
    }

    let res = LnurlWellKnownResponse {
        callback: "http://localhost:3000/lnurlp/kody/callback".parse()?,
        max_sendable: "10000".parse()?,
        min_sendable: "1000".parse()?,
        metadata: "[]".to_string(),
        comment_allowed: None,
        tag: crate::models::LnurlType::PayRequest,
        status: crate::models::LnurlStatus::Ok,
        nostr_pubkey: None,
        allows_nostr: false,
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

    let pr = state
        .fm_client
        .get_first_module::<LightningClientModule>()
        .create_bolt11_invoice(Amount { msats: 1000 }, "test invoice".to_string(), None, ())
        .await?;

    let verify_url = format!(
        "http://localhost:3000/lnurlp/kody/verify/{}",
        pr.0.to_string()
    );

    let res = LnurlCallbackResponse {
        pr: pr.1.to_string(),
        success_action: None,
        status: LnurlStatus::Ok,
        reason: None,
        verify: verify_url.parse()?,
        routes: None,
    };

    Ok(Json(res))
}

#[axum_macros::debug_handler]
pub async fn lnurlp_verify(username: String) -> String {
    format!("lnurlp_verify stub for {}", username)
}
