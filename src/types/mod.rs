pub mod lnurl;
pub mod nostr;

pub enum NameOrPubkey {
    Name,
    #[allow(dead_code)]
    Pubkey,
}
