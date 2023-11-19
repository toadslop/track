use once_cell::sync::Lazy;
use track_api_challenge::configuration::get_configuration;
use track_api_challenge::telemetry::init_telemetry;

pub static TRACING: Lazy<anyhow::Result<()>> = Lazy::new(|| {
    let configuration = get_configuration().expect("Failed to read configuration");
    init_telemetry(&configuration.telemetry)?;

    Ok(())
});
