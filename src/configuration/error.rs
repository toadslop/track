use config::ConfigError;
use std::{env::VarError, io};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigurationError {
    #[error("Failed to parse environment from value '{0}'")]
    ParseEnvFailed(String),
    #[error("Failed to locate execution directory of binary")]
    CurDirNotFound(io::Error),
    #[error("Failed to read environment variable")]
    EnvVarReadFailures(#[from] VarError),
    #[error("Failed to initialize the configuration")]
    ConfigInitFail(#[from] ConfigError),
}
