use std::str::FromStr;

use axum::{
    extract::{Path, State},
    Json,
};
use fedimint_core::core::OperationId;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::{error::AppError, model::invoice::InvoiceBmc, state::AppState};

use super::LnurlStatus;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LnurlVerifyResponse {
    pub status: LnurlStatus,
    pub settled: bool,
    pub preimage: String,
    pub pr: String,
}

#[axum_macros::debug_handler]
pub async fn handle_verify(
    Path((username, op_id)): Path<(String, String)>,
    State(state): State<AppState>,
) -> Result<Json<LnurlVerifyResponse>, AppError> {
    info!(
        "verify called with username: {}, op_id: {}",
        username, op_id
    );

    // Use the operation id to look up the invoice
    let invoice = InvoiceBmc::get_by_op_id(&state.mm, &op_id).await?;

    let verify_response = LnurlVerifyResponse {
        status: LnurlStatus::Ok,
        settled: invoice.settled,
        preimage: "".to_string(), // TODO: figure out how to get the preimage from fedimint client
        pr: invoice.bolt11,
    };

    Ok(Json(verify_response))
}
