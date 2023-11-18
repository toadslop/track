use track_api_challenge::{
    app::Application, configuration::get_configuration, telemetry::init_telemetry,
};

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    init_telemetry()?;
    let config = get_configuration()?;
    let app = Application::build(config).await?;
    app.run_until_stopped().await?;

    Ok(())
}
