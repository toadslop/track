use once_cell::sync::Lazy;
use track_api_challenge::telemetry::init_telemetry;

pub static TRACING: Lazy<anyhow::Result<()>> = Lazy::new(|| {
    init_telemetry()?;

    Ok(())
});
