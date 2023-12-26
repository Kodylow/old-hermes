use std::fs::read_to_string;

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
