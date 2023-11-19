use once_cell::sync::Lazy;
use track_api_challenge::configuration;
use track_api_challenge::telemetry;

pub static TRACING: Lazy<anyhow::Result<()>> = Lazy::new(|| {
    let configuration = configuration::init().expect("Failed to read configuration");
    telemetry::init(&configuration.telemetry)?;

    Ok(())
});
