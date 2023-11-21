use track_api_challenge::{app::Application, configuration, database, telemetry};

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    telemetry::init()?;
    let config = configuration::init()?;
    let db = database::init(&config.database).await?;
    let app = Application::build(config, db).await?;

    tracing::info!("App is running on port {}", app.port());
    app.run_until_stopped().await?;

    Ok(())
}
