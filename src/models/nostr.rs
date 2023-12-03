use std::collections::HashMap;

use nostr::prelude::XOnlyPublicKey;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct RegisterParams {
    pub username: Option<String>,
    pub nostr_pubkey: XOnlyPublicKey,
}

#[derive(Deserialize)]
pub struct Nip05WellKnownParams {
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Nip05WellKnown {
    pub names: HashMap<String, XOnlyPublicKey>,
    pub relays: HashMap<XOnlyPublicKey, Vec<String>>,
}
