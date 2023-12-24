use anyhow::Result;
use tracing::info;

mod config;
mod error;
mod models;
mod router;
mod state;

mod utils;
use state::{get_nostr_json, load_fedimint_client, AppState};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let state = AppState {
        fm_client: load_fedimint_client().await?,
        nostr_json: get_nostr_json(),
    };

    let app = router::create_router(state).await?;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("Listening on 0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
