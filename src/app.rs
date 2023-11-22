use crate::{
    configuration::{application::ApplicationSettings, auth::AuthSettings, Settings},
    database::Database,
    domain::user::actions::SignupError,
    error::ErrorResponse,
    routes::{private::private_services, public::public_services},
};
use actix_web::{
    dev::Server,
    error::InternalError,
    web::{self, JsonConfig},
    App, HttpResponse, HttpServer,
};
use std::{fmt::Debug, net::TcpListener};

/// A wrapper for the actix instance. It hides the details of the actix instance
/// and only exposes functionality that we need elsewhere.
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
    /// Takes the configuration and the database and builds the application, but does not
    /// run it.
    #[tracing::instrument(name = "build_app")]
    pub async fn build(configuration: Settings, db: Database) -> anyhow::Result<Self> {
        tracing::debug!("Building application");

        let mut settings = configuration.application;
        tracing::debug!("settings: {settings:?}");

        let address = format!("{}:{}", &settings.host, &settings.port);
        tracing::debug!("app address: {address}");

        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr()?.port();
        settings.port = port;

        let server = Self::build_actix_instance(listener, db, configuration.auth).await?;

        Ok(Self { settings, server })
    }

    /// Contains the logic for assembling and running the actix-web instance. This
    /// function should receive everything it needs to run the app -- it should not
    /// initialize anything other than the actix instance.
    async fn build_actix_instance(
        listener: TcpListener,
        db: Database,
        auth_settings: AuthSettings,
    ) -> anyhow::Result<Server> {
        let db = web::Data::new(db);
        let auth_settings = web::Data::new(auth_settings);
        let json_cfg = Self::init_json_config();

        let server = HttpServer::new(move || {
            App::new()
                .configure(private_services)
                .configure(public_services)
                .app_data(db.clone())
                .app_data(auth_settings.clone())
                .app_data(json_cfg.clone())
        })
        .listen(listener)?
        .run();

        Ok(server)
    }

    /// Expose the application port. Useful for times when the port was generated.
    pub fn port(&self) -> u16 {
        self.settings.port
    }

    /// Start the application and run on an infinite loop.
    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }

    fn init_json_config() -> JsonConfig {
        web::JsonConfig::default().error_handler(|err, req| {
            let error = match err {
                actix_web::error::JsonPayloadError::Deserialize(e) => {
                    if req.path().ends_with("/signup") {
                        println!("HERE");
                        InternalError::from_response(
                            e,
                            HttpResponse::BadRequest()
                                .json(ErrorResponse::from(&SignupError::InvalidPayload)),
                        )
                    } else {
                        let message = e.to_string();
                        InternalError::from_response(e, HttpResponse::BadRequest().json(message))
                    }
                }
                _ => return err.into(),
            };

            error.into()
        })
    }
}
