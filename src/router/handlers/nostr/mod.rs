use serde::{Deserialize, Serialize};

pub mod register;
pub mod well_known;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Nip05Relays {
    pub pubkey: String,
    pub name: String,
    pub relays: Vec<String>,
}
