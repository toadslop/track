use crate::{
    configuration::{application::ApplicationSettings, Settings},
    run,
};
use actix_web::dev::Server;
use std::{fmt::Debug, net::TcpListener};

pub struct Application {
    settings: ApplicationSettings,
    server: Server,
}

impl Debug for Application {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Application")
            .field("settings", &self.settings)
            .field("server", &"actix_web::dev::Server")
            .finish()
    }
}

impl Application {
    #[tracing::instrument(name = "build_app")]
    pub async fn build(configuration: Settings) -> anyhow::Result<Self> {
        tracing::debug!("Building application");
        let mut settings = configuration.application;
        tracing::debug!("settings: {settings:?}");
        let address = format!("{}:{}", &settings.host, &settings.port);

        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr()?.port();
        settings.port = port;

        let server = run(listener).await?;

        Ok(Self { settings, server })
    }

    pub fn port(&self) -> u16 {
        self.settings.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}
