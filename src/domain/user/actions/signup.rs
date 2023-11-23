use crate::{
    auth::hash_password,
    database::Database,
    domain::user::{
        dto::{self, SignupResponse},
        User,
    },
};
use argon2::password_hash::{self};
use chrono::Utc;
use thiserror::Error;
use uuid::Uuid;

/// Performs the necessary procedures required for signing up a new user.
#[tracing::instrument]
pub async fn signup(db: &Database, user_dto: dto::Signup) -> Result<SignupResponse, SignupError> {
    tracing::debug!("Validating request integrity...");
    let (password, user_id) = match (user_dto.password, user_dto.user_id) {
        (Some(password), Some(user_id)) => (password, user_id),
        _ => Err(SignupError::InvalidPayload)?,
    };
    tracing::debug!("Request contains required fields");

    tracing::debug!("Checking if user already exists...");
    let user = sqlx::query_as::<_, User>("SELECT * FROM user_ WHERE user_id = $1")
        .bind(&user_id)
        .fetch_optional(db.inner())
        .await?;

    if user.is_some() {
        Err(SignupError::UserAlreadyExists(user_id.to_owned()))?;
    }
    tracing::debug!("User does not exist");

    tracing::debug!("Hashing password");
    let hashed_password = hash_password(&password).map_err(SignupError::PasswordHash)?;
    tracing::debug!("Password hash success");

    tracing::debug!("Inserting user into DB");
    let user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO user_ (id, user_id, password, created_at, nickname)
        VALUES($1, $2, $3, $4, $5)
        RETURNING user_id, nickname, id, password;
    "#,
    )
    .bind(Uuid::new_v4())
    .bind(&user_id)
    .bind(hashed_password)
    .bind(Utc::now())
    .bind(&user_id) // DEFAULT
    .fetch_one(db.inner())
    .await?;
    tracing::debug!("Insert user success");

    Ok(user.into())
}

#[derive(Debug, Error)]
pub enum SignupError {
    #[error("Failed to hash password")]
    PasswordHash(password_hash::Error),
    #[error("Error when persisting user: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Failed to encode the jwt")]
    JwtError(#[from] jsonwebtoken::errors::Error),
    #[error("Invalid payload received")]
    InvalidPayload,
    #[error("A user with id {0} already exists")]
    UserAlreadyExists(String),
}
