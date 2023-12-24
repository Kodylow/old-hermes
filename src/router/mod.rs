use anyhow::Result;
use axum::{
    routing::{get, post},
    Router,
};
pub mod handlers;
pub mod state;

use handlers::*;
use state::AppState;

use self::state::{get_nostr_json, load_fedimint_client};

pub async fn create_router() -> Result<Router> {
    let state = AppState {
        fm_client: load_fedimint_client().await?,
        nostr_json: get_nostr_json(),
    };

    let app = Router::new()
        .route("/", get(handle_readme))
        .route("/health", get(|| async { "OK" }))
        .route("/register", post(register))
        .route("/.well-known/nostr.json", get(nostr::nip05_well_known))
        .route("/.well-known/lnurlp/:username", get(lnurlp::well_known))
        .route("/lnurlp/:username/callback", get(lnurlp::callback))
        .route(
            "/lnurlp/:username/verify/:operation_id",
            get(lnurlp::verify),
        )
        .with_state(state);

    Ok(app)
}
