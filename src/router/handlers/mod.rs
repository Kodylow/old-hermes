use std::fmt;
use std::fs::read_to_string;

use serde::{Deserialize, Serialize};

pub mod lnurlp;
pub mod nostr;

#[axum_macros::debug_handler]
pub async fn handle_readme() -> String {
    read_to_string("README.md").expect("Could not read README.md")
}

pub enum NameOrPubkey {
    Name,
    #[allow(dead_code)]
    Pubkey,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SupportedDmType {
    Nostr,
    Xmpp,
}

impl fmt::Display for SupportedDmType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SupportedDmType::Nostr => write!(f, "nostr"),
            SupportedDmType::Xmpp => write!(f, "xmpp"),
        }
    }
}
