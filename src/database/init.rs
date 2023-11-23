use super::error::DatabaseInitError;
use super::Database;
use crate::configuration::database::DatabaseSettings;
use crate::domain::user::actions::signup;
use crate::domain::user::dto::Signup;
use secrecy::Secret;
use sqlx::migrate::MigrateDatabase;
use sqlx::PgPool;
use std::env;
use std::path::Path;
use std::time::Duration;

/// Initializes the database with the settings derived from the config file and
/// environment variables. This function will wait for the database to become available,
/// but will eventually time out. It will attempt to connect for 5 times by default
/// and will wait 1 second between attempts by default, but this is configurable using
/// the config file or environment variables.
///
/// Configuration options are as follows:
///
///
/// `port`
/// `host`
/// `password`
/// `user`
/// `name`
/// `init_wait_interval`
/// `init_wait_retry_count`
///
///
/// Note that, for local development, environment variables should be used to configure
/// the port, host, password, user, and name of the database as these need to be
/// initialized when the database docker container starts up. Create a .env file
/// and add the values as follows:
///
///
/// `TRACK__DATABASE_USER=user`
/// `TRACK__DATABASE_PASSWORD=password`
/// `TRACK__DATABASE_NAME=track`
/// `TRACK__DATABASE_PORT=5433`
/// `TRACK__DATABASE_HOST=localhost`
///
#[tracing::instrument(name = "init_database")]
pub async fn init(settings: &DatabaseSettings) -> Result<Database, DatabaseInitError> {
    tracing::info!("Initializing database with settings: {settings:?}");

    let db_url = settings.connection_string();

    wait_for_db_connection(
        &db_url,
        settings.init_wait_retry_count,
        settings.init_wait_interval,
    )
    .await?;

    if !sqlx::Postgres::database_exists(&db_url).await? {
        tracing::trace!("Creating database");
        sqlx::Postgres::create_database(&db_url).await?;
    }

    tracing::trace!("Connecting to database");
    let db = PgPool::connect(&db_url).await?;
    tracing::trace!("Connect success");

    let migrations = if env::var("RUST_ENV") == Ok("production".to_string()) {
        tracing::debug!("Loading production migrations");
        std::env::current_exe()?.join("./migrations")
    } else {
        tracing::debug!("Loading development migrations");
        let crate_dir = std::env::var("CARGO_MANIFEST_DIR")?;
        Path::new(&crate_dir).join("./migrations")
    };

    tracing::info!("Running migrations...");
    sqlx::migrate::Migrator::new(migrations)
        .await?
        .run(&db)
        .await?;
    tracing::info!("Migrations success");
    let db: Database = Database::from(db);

    match signup(
        &db,
        Signup {
            user_id: Some("TaroYamada".into()),
            password: Some(Secret::new("PaSSwd4TY".into())),
        },
    )
    .await
    {
        Ok(_) => tracing::info!("Created test user"),
        Err(_) => tracing::warn!("Test user already exists"),
    };

    Ok(db)
}

async fn wait_for_db_connection(
    db_url: &str,
    max_retries: u8,
    retry_interval: u64,
) -> Result<(), DatabaseInitError> {
    let mut retry_count = 0;
    let err = loop {
        tracing::info!("Waiting for db (count: {retry_count})");
        match sqlx::Postgres::database_exists(db_url).await {
            Ok(_) => {
                tracing::info!("Connected to db");
                return Ok(());
            }
            Err(e) => {
                if retry_count >= max_retries {
                    break e;
                };
                actix_web::rt::time::sleep(Duration::from_millis(retry_interval)).await
            }
        }

        retry_count += 1;
    };

    Err(DatabaseInitError::ConnectionFailure(err))
}
