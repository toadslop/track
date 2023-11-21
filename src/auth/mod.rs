use argon2::{
    password_hash::{self, rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use secrecy::{ExposeSecret, Secret};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

pub fn verify_jwt(password: &str, submitted_password: &Secret<String>) -> Result<(), JwtError> {
    tracing::debug!("Generating has from db user's password");
    let hash = PasswordHash::new(password).map_err(JwtError::PasswordHash)?;
    tracing::debug!("Success");

    tracing::debug!("Verifying password...");
    Argon2::default()
        .verify_password(submitted_password.expose_secret().as_bytes(), &hash)
        .map_err(JwtError::InvalidCredentials)?;
    tracing::debug!("Jwt is valid");
    Ok(())
}

pub fn issue_jwt(user_id: &Uuid, jwt_secret: &Secret<String>) -> Result<String, JwtError> {
    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::minutes(60)).timestamp() as usize; // TODO: make this configurable
    let claims = TokenClaims {
        sub: user_id.to_string(),
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

pub fn hash_password(password: &Secret<String>) -> Result<String, password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    Ok(Argon2::default()
        .hash_password(password.expose_secret().as_bytes(), &salt)?
        .to_string())
}

#[derive(Debug, Error)]
pub enum JwtError {
    #[error("Failed to hash password: {0}")]
    PasswordHash(password_hash::Error),
    #[error("User attempted to signin with invalid credentials: {0}")]
    InvalidCredentials(argon2::password_hash::Error),
    #[error("Failed to encode the jwt: {0}")]
    JwtError(#[from] jsonwebtoken::errors::Error),
}
