mod app;
mod config;
mod db;
mod http;
mod modules;
mod openapi;
mod state;

use anyhow::Context;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    orbit_telemetry::init();

    let config = config::ApiConfig::from_env();
    let state = state::AppState::new(config.clone());
    let app = app::build(state);
    let address = config.server_address();

    let listener = TcpListener::bind(&address)
        .await
        .with_context(|| format!("failed to bind API server to {address}"))?;

    tracing::info!(%address, "starting Orbit API");
    axum::serve(listener, app)
        .await
        .context("API server failed")
}
