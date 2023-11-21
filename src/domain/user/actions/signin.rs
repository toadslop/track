use argon2::{password_hash, Argon2, PasswordHash, PasswordVerifier};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use secrecy::{ExposeSecret, Secret};
use thiserror::Error;

use crate::{
    database::Database,
    domain::user::{dto, User},
    middleware::auth::TokenClaims,
};

#[tracing::instrument]
pub async fn signin(
    db: &Database,
    user_info: &dto::Signin,
    jwt_secret: &Secret<String>,
) -> Result<String, SigninError> {
    tracing::debug!(
        "Requesting user from db where email is {}",
        &user_info.email
    );
    let user = sqlx::query_as::<_, User>("SELECT * FROM user_ WHERE email = $1")
        .bind(&user_info.email)
        .fetch_optional(db.inner())
        .await?
        .ok_or(SigninError::UserNotFound)?;
    tracing::debug!("User found");

    tracing::debug!("Generating has from db user's password");
    let hash = PasswordHash::new(&user.password).map_err(SigninError::PasswordHash)?;
    tracing::debug!("Success");

    tracing::debug!("Verifying password...");
    Argon2::default()
        .verify_password(user_info.password.expose_secret().as_bytes(), &hash)
        .map_err(SigninError::InvalidCredentials)?;
    tracing::debug!("Verifying password...");

    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::minutes(60)).timestamp() as usize;
    let claims: TokenClaims = TokenClaims {
        sub: user.id.to_string(),
        exp,
        iat,
    };

    tracing::debug!("Encoding JWT...");
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.expose_secret().as_ref()),
    )?;
    tracing::debug!("Encoding success");

    Ok(token)
}

#[derive(Debug, Error)]
pub enum SigninError {
    #[error("No user with the provided password and email combination was found")]
    UserNotFound,
    #[error("Failed to hash password")]
    PasswordHash(password_hash::Error),
    #[error("Error when persisting user: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Failed to encode the jwt")]
    JwtError(#[from] jsonwebtoken::errors::Error),
    #[error("User attempted to signin with invalid credentials")]
    InvalidCredentials(argon2::password_hash::Error),
}
