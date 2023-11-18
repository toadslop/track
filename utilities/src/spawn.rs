use super::{telemetry::TRACING, test_app::TestApp};
use actix_web::rt::spawn;
use once_cell::sync::Lazy;
use std::env;
use track_api_challenge::app::Application;
use track_api_challenge::configuration::{get_app_env_key, get_configuration};

pub async fn spawn_app() -> anyhow::Result<TestApp> {
    env::set_var(get_app_env_key(), "test");

    Lazy::force(&TRACING);

    let mut configuration = get_configuration().expect("Failed to read configuration");
    configuration.application.port = 0;
    let application = Application::build(configuration).await?;

    let app_address =
        reqwest::Url::parse(&format!("http://127.0.0.1:{}", application.port())).unwrap();

    spawn(application.run_until_stopped());

    Ok(TestApp::new(app_address))
}
