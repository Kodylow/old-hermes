use serde::{Deserialize, Serialize};

pub mod callback;
pub mod verify;
pub mod well_known;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LnurlType {
    PayRequest,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum LnurlStatus {
    Ok,
    Error,
}
