use track_api_challenge::{
    app::Application, configuration::get_configuration, telemetry::init_telemetry,
};

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let config = get_configuration()?;
    init_telemetry(&config.telemetry)?;

    let app = Application::build(config).await?;

    tracing::info!("App is running on port {}", app.port());
    app.run_until_stopped().await?;

    Ok(())
}
