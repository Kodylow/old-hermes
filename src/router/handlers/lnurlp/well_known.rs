use axum::extract::{Path, State};
use axum::Json;
use fedimint_core::Amount;
use nostr::prelude::XOnlyPublicKey;
use serde::ser::{SerializeTuple, Serializer};
use serde::{Deserialize, Serialize};
use tracing::info;
use url::Url;

use super::{LnurlStatus, LnurlType};
use crate::config::CONFIG;
use crate::error::AppError;
use crate::model::app_user::AppUserBmc;
use crate::router::handlers::NameOrPubkey;
use crate::state::AppState;

#[derive(Serialize, Deserialize, Debug)]
pub enum MetadataType {
    TextPlain,
    ImagePngBase64,
    ImageJpegBase64,
    TextEmail,
    TextIdentifier,
}

#[derive(Deserialize)]
pub struct MetadataEntry {
    pub metadata_type: MetadataType,
    pub content: String,
}

impl Serialize for MetadataEntry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut tup = serializer.serialize_tuple(2)?;
        tup.serialize_element(&format!("{:?}", self.metadata_type))?;
        tup.serialize_element(&self.content)?;
        tup.end()
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LnurlWellKnownResponse {
    pub callback: Url,
    pub max_sendable: Amount,
    pub min_sendable: Amount,
    pub metadata: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment_allowed: Option<i32>,
    pub tag: LnurlType,
    pub status: LnurlStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nostr_pubkey: Option<XOnlyPublicKey>,
    pub allows_nostr: bool,
}

#[axum_macros::debug_handler]
pub async fn handle_well_known(
    Path(username): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<LnurlWellKnownResponse>, AppError> {
    // see if username exists in nostr.json
    info!("well_known called with username: {}", username);
    let _app_user = AppUserBmc::get_by(&state.mm, NameOrPubkey::Name, &username).await?;

    let res = LnurlWellKnownResponse {
        callback: format!("http://{}/lnurlp/{}/callback", CONFIG.domain, username).parse()?,
        max_sendable: Amount { msats: 100000 },
        min_sendable: Amount { msats: 1000 },
        metadata: "test metadata".to_string(),
        comment_allowed: None,
        tag: LnurlType::PayRequest,
        status: LnurlStatus::Ok,
        nostr_pubkey: Some(CONFIG.nostr_sk.public_key()),
        allows_nostr: true,
    };

    Ok(Json(res))
}
