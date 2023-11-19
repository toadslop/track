use self::{application::ApplicationSettings, jaeger::JaegerSettings};
use crate::configuration::{environment::Environment, error::ConfigurationError};
use config::{Config, FileFormat};
use dotenv::dotenv;
use serde::Deserialize;

pub mod application;
mod environment;
mod error;
pub mod jaeger;
pub mod scheme;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub telemetry: JaegerSettings,
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
pub fn get_configuration() -> Result<Settings, ConfigurationError> {
    tracing::debug!("Loading configuration");

    dotenv().ok();

    let base_path = std::env::current_dir().map_err(ConfigurationError::CurDirNotFound)?;
    tracing::trace!("base path: {:?}", base_path);

    let configuration_directory = base_path.join(CONFIG_DIR_NAME);
    tracing::trace!("config directory: {:?}", configuration_directory);

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

    tracing::trace!("App environment: {environment}");

    let environment_filename = format!("config.{}.yaml", environment.as_ref());

    tracing::trace!("Environment filename: {environment_filename}");

    let settings = Config::builder()
        .set_default("application.port", ApplicationSettings::default().port)?
        .set_default("application.host", ApplicationSettings::default().host)?
        .set_default("application.scheme", ApplicationSettings::default().scheme)?
        .set_default("application.domain", ApplicationSettings::default().domain)?
        .set_default("telemetry.port", JaegerSettings::default().port)?
        .set_default("telemetry.scheme", JaegerSettings::default().scheme)?
        .set_default("telemetry.host", JaegerSettings::default().host)?
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

    tracing::trace!("settings {:?}", settings);

    let settings = settings.try_deserialize::<Settings>()?;

    tracing::debug!("Configuration loaded");

    Ok(settings)
}
