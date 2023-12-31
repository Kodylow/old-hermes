use anyhow::Result;
use axum::{
    routing::{get, post},
    Router,
};
pub mod handlers;

use handlers::*;

use crate::state::AppState;

pub async fn create_router(state: AppState) -> Result<Router> {
    let app = Router::new()
        .route("/", get(handle_readme))
        .route("/health", get(|| async { "OK" }))
        .route("/register", post(nostr::register::handle_register))
        .route(
            "/.well-known/nostr.json",
            get(nostr::well_known::handle_nip05_well_known),
        )
        .route(
            "/.well-known/lnurlp/:username",
            get(lnurlp::well_known::handle_well_known),
        )
        .route(
            "/lnurlp/:username/callback",
            get(lnurlp::callback::handle_callback),
        )
        .route(
            "/lnurlp/:username/verify/:op_id",
            get(lnurlp::verify::handle_verify),
        )
        .with_state(state);

    Ok(app)
}
