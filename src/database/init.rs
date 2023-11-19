use crate::configuration::database::DatabaseSettings;
use sqlx::migrate::MigrateDatabase;
use sqlx::PgPool;
use std::env;
use std::path::Path;

static DATABASE_URL: &str = "DATABASE_URL";

#[tracing::instrument]
pub async fn init(settings: &DatabaseSettings) -> anyhow::Result<PgPool> {
    tracing::info!("Initializing database: {settings:?}");
    // Use the existing DATABASE_URL env var if exists; otherwise, use the connection string
    // from the settings file
    let db_url = match env::var(DATABASE_URL) {
        Ok(url) => url,
        Err(e) => match e {
            env::VarError::NotPresent => {
                env::set_var(DATABASE_URL, settings.connection_string());
                env::var(DATABASE_URL)?
            }
            _ => Err(e)?,
        },
    };

    tracing::debug!(db_url);

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

    Ok(db)
}
