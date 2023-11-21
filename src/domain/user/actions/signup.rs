use crate::{
    auth::hash_password,
    database::Database,
    domain::user::{dto, User},
};
use argon2::password_hash::{self};
use chrono::Utc;
use thiserror::Error;
use uuid::Uuid;

/// Performs the necessary procedures required for signing up a new user.
#[tracing::instrument]
pub async fn signup(db: &Database, user_dto: &dto::Signup) -> Result<User, SignupError> {
    tracing::debug!("Hashing password");
    let hashed_password = hash_password(&user_dto.password).map_err(SignupError::PasswordHash)?;
    tracing::debug!("Password hash success");

    tracing::debug!("Inserting user into DB");
    sqlx::query(
        r#"
        INSERT INTO user_ (id, email, name, password, created_at)
        VALUES($1, $2, $3, $4, $5)
    "#,
    )
    .bind(Uuid::new_v4())
    .bind(&user_dto.email)
    .bind(&user_dto.name)
    .bind(hashed_password)
    .bind(Utc::now())
    .execute(db.inner())
    .await?;
    tracing::debug!("Insert user success");

    tracing::debug!("Retrieving user info from DB");
    let user = sqlx::query_as::<_, User>("SELECT * FROM user_ WHERE email = $1")
        .bind(&user_dto.email)
        .fetch_one(db.inner())
        .await?;
    tracing::debug!("Retrieval success");

    Ok(user)
}

#[derive(Debug, Error)]
pub enum SignupError {
    #[error("Failed to hash password")]
    PasswordHash(password_hash::Error),
    #[error("Error when persisting user: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Failed to encode the jwt")]
    JwtError(#[from] jsonwebtoken::errors::Error),
}
