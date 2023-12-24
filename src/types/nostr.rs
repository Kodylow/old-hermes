use std::collections::HashMap;

use nostr::prelude::XOnlyPublicKey;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Nip05WellKnown {
    pub names: HashMap<String, XOnlyPublicKey>,
    pub relays: HashMap<XOnlyPublicKey, Vec<String>>,
}
