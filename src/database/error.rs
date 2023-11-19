use std::{env::VarError, io};

use sqlx::migrate::MigrateError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DatabaseInitError {
    #[error("Failed to initialize the database")]
    SqlxError(#[from] sqlx::Error),
    #[error("Encountered issue reading environment variables")]
    EnvironmentVar(#[from] VarError),
    #[error("Encountered file system IO error")]
    Io(#[from] io::Error),
    #[error("Failed to run migration")]
    MigrationError(#[from] MigrateError),
    #[error("Failed to connect to the database")]
    ConnectionFailure(sqlx::Error),
}
