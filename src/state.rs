use fedimint_client::ClientArc;

use crate::models::nostr::Nip05WellKnown;

#[derive(Debug, Clone)]
pub struct AppState {
    pub fm_client: ClientArc,
    pub nostr_json: Nip05WellKnown,
}
