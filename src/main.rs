use anyhow::Result;
use axum::{
    routing::{get, post},
    Router,
};
use dotenv::dotenv;
use fedimint_lnurl::{
    load_fedimint_client,
    routes::{handle_readme, lnurlp_callback, lnurlp_verify, register, well_known},
};
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let fedimint_client = load_fedimint_client().await?;
    info!("Loaded fedimint client");
    info!(
        "Fedimint client info: {:?}",
        fedimint_client.federation_id()
    );

    let app = Router::new()
        .route("/", get(handle_readme))
        .route("/health", get(|| async { "OK" }))
        .route("/register", post(register))
        .route("/.well-known/lnurlp/{username}", get(well_known))
        .route("/lnurlp/{username}/callback", get(lnurlp_callback))
        .route("/lnurlp/{username}/verify", get(lnurlp_verify));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("Listening on 0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
