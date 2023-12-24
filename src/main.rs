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

use crate::model::ModelManager;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let state = AppState {
        fm: load_fedimint_client().await?,
        mm: ModelManager::new().await?,
    };

    let app = router::create_router(state).await?;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("Listening on 0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
