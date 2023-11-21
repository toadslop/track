use self::application::ApplicationSettings;
use crate::configuration::{
    auth::AuthSettings, database::DatabaseSettings, environment::Environment,
    error::ConfigurationError,
};
use config::{Config, FileFormat};
use dotenv::dotenv;
use secrecy::ExposeSecret;
use serde::Deserialize;

pub mod application;
pub mod auth;
pub mod database;
mod environment;
mod error;
pub mod scheme;

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub database: DatabaseSettings,
    pub auth: AuthSettings,
}

const APP_ENV_KEY: &str = "ENVIRONMENT";
const APP_ENV_PREFIX: &str = "TRACK";
const APP_ENV_PREFIX_SEP: &str = "__";
const BASE_CONFIG_FILENAME: &str = "config.yaml";
const SEP: &str = "_";
const CONFIG_DIR_NAME: &str = "config";
pub fn get_app_env_key() -> String {
    format!("{APP_ENV_PREFIX}{APP_ENV_PREFIX_SEP}APPLICATION{SEP}{APP_ENV_KEY}")
}

#[tracing::instrument]
pub fn init() -> Result<Settings, ConfigurationError> {
    tracing::info!("Loading configuration");

    dotenv().ok();

    let base_path = std::env::current_dir().map_err(ConfigurationError::CurDirNotFound)?;
    tracing::debug!("base path: {:?}", base_path);

    let configuration_directory = base_path.join(CONFIG_DIR_NAME);
    tracing::debug!("config directory: {:?}", configuration_directory);

    let app_env_key = get_app_env_key();

    let environment: Environment = match std::env::var(&app_env_key) {
        Ok(env) => env.try_into()?,
        Err(e) => match e {
            std::env::VarError::NotPresent => {
                std::env::set_var(&app_env_key, Environment::default().as_ref());
                Environment::default()
            }
            _ => Err(e)?,
        },
    };

    tracing::info!("App environment: {environment}");

    let environment_filename = format!("config.{}.yaml", environment.as_ref());

    tracing::debug!("Environment filename: {environment_filename}");

    let settings = Config::builder()
        .set_default("application.port", ApplicationSettings::default().port)?
        .set_default("application.host", ApplicationSettings::default().host)?
        .set_default("application.scheme", ApplicationSettings::default().scheme)?
        .set_default("application.domain", ApplicationSettings::default().domain)?
        .set_default("database.port", DatabaseSettings::default().port)?
        .set_default("database.name", DatabaseSettings::default().name)?
        .set_default("database.host", DatabaseSettings::default().host)?
        .set_default(
            "database.init_wait_interval",
            DatabaseSettings::default().init_wait_interval,
        )?
        .set_default(
            "database.init_wait_retry_count",
            DatabaseSettings::default().init_wait_retry_count,
        )?
        .set_default(
            "database.password",
            DatabaseSettings::default()
                .password
                .expose_secret()
                .as_str(),
        )?
        .set_default("database.user", DatabaseSettings::default().user)?
        .set_default("auth.jwt_max_age", AuthSettings::default().jwt_max_age)?
        .set_default(
            "auth.jwt_expires_in",
            AuthSettings::default().jwt_expires_in,
        )? // Note: we don't allow a default for the secret for security reasons
        .add_source(
            config::File::from(configuration_directory.join(BASE_CONFIG_FILENAME))
                .required(false)
                .format(FileFormat::Yaml),
        )
        .add_source(
            config::File::from(configuration_directory.join(environment_filename))
                .required(false)
                .format(FileFormat::Yaml),
        )
        .add_source(
            config::Environment::with_prefix(APP_ENV_PREFIX)
                .prefix(APP_ENV_PREFIX)
                .prefix_separator(APP_ENV_PREFIX_SEP)
                .separator(SEP),
        );

    let settings = settings.build()?;

    tracing::info!("Loaded with settings {:?}", settings);

    let settings = settings.try_deserialize::<Settings>()?;

    tracing::info!("Configuration load successful");

    Ok(settings)
}
