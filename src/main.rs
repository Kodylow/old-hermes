use anyhow::Result;
use tracing::info;

mod config;
mod error;
mod model;
mod router;
mod state;
mod types;

mod utils;
use state::{load_fedimint_client, AppState};

use crate::{config::CONFIG, model::ModelManager, state::load_nostr_client};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let state = AppState {
        fm: load_fedimint_client().await?,
        mm: ModelManager::new().await?,
        nostr: load_nostr_client().await?,
    };

    let app = router::create_router(state).await?;

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", CONFIG.domain, CONFIG.port))
        .await
        .unwrap();
    info!("Listening on {}", CONFIG.port);
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
