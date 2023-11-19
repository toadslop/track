use track_api_challenge::{app::Application, configuration, database, telemetry};

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let config = configuration::init()?;
    telemetry::init(&config.telemetry)?;
    database::init(&config.database).await?;

    let app = Application::build(config).await?;

    tracing::info!("App is running on port {}", app.port());
    app.run_until_stopped().await?;

    Ok(())
}
