mod jobs;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    orbit_telemetry::init();
    tracing::info!("starting Orbit worker");

    Ok(())
}
