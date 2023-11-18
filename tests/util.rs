use actix_web::rt::spawn;
use once_cell::sync::Lazy;
use std::env;
use track_api_challenge::{
    app::Application,
    configuration::{get_app_env_key, get_configuration},
    telemetry::init_telemetry,
};

static TRACING: Lazy<anyhow::Result<()>> = Lazy::new(|| {
    init_telemetry()?;

    Ok(())
});

pub struct TestApp {
    pub app_address: reqwest::Url,
}

impl TestApp {
    pub async fn get_hello_world(&self) -> reqwest::Response {
        reqwest::Client::new()
            .get(self.app_address.join("/").unwrap())
            .send()
            .await
            .expect("failed to execute request")
    }
}

pub async fn spawn_app() -> anyhow::Result<TestApp> {
    env::set_var(get_app_env_key(), "test");

    Lazy::force(&TRACING);

    let configuration = get_configuration().expect("Failed to read configuration");
    let application = Application::build(configuration).await?;

    let app_address =
        reqwest::Url::parse(&format!("http://127.0.0.1:{}", application.port())).unwrap();

    spawn(application.run_until_stopped());

    Ok(TestApp { app_address })
}
