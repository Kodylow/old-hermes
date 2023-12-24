use std::collections::HashMap;

use nostr::prelude::XOnlyPublicKey;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Nip05Relays {
    pub pubkey: String,
    pub name: String,
    pub relays: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Nip05WellKnownParams {
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Nip05WellKnown {
    pub names: HashMap<String, XOnlyPublicKey>,
    pub relays: HashMap<XOnlyPublicKey, Vec<String>>,
}

impl Nip05WellKnown {
    pub fn from_db(nip05relays: Nip05Relays) -> Self {
        let mut names = HashMap::new();
        names.insert(
            nip05relays.name,
            XOnlyPublicKey::from_str(&nip05relays.pubkey).unwrap(),
        );
        let mut relays = HashMap::new();
        relays.insert(
            XOnlyPublicKey::from_str(&nip05relays.pubkey).unwrap(),
            nip05relays.relays,
        );
        Self { names, relays }
    }
}
