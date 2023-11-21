use track_api_challenge::anyhow;
use track_api_challenge::once_cell::sync::Lazy;
use track_api_challenge::telemetry;

pub static TRACING: Lazy<anyhow::Result<()>> = Lazy::new(|| {
    telemetry::init()?;

    Ok(())
});
