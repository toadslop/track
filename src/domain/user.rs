use crate::database::Database;
use crate::middleware::auth::TokenClaims;
use argon2::password_hash;
use argon2::PasswordHash;
use argon2::PasswordHasher;
use argon2::PasswordVerifier;
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2,
};
use chrono::Duration;
use chrono::Utc;
use jsonwebtoken::encode;
use jsonwebtoken::EncodingKey;
use jsonwebtoken::Header;
use secrecy::ExposeSecret;
use secrecy::Secret;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateUserDto {
    pub name: String,
    pub email: String,
    pub password: Secret<String>,
}

#[derive(Debug, Deserialize)]
pub struct LoginUserDto {
    pub email: String,
    pub password: Secret<String>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct User {
    pub name: String,
    pub email: String,
    pub password: String,
    pub id: Uuid,
}

impl Database {
    #[tracing::instrument]
    pub async fn insert_user(&self, user_dto: &CreateUserDto) -> Result<User, UserError> {
        tracing::debug!("Generating salt");
        let salt = SaltString::generate(&mut OsRng);

        tracing::debug!("Hashing password");
        let hashed_password = Argon2::default()
            .hash_password(user_dto.password.expose_secret().as_bytes(), &salt)
            .map_err(UserError::PasswordHashError)?
            .to_string();

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
        .execute(self.inner())
        .await?;
        tracing::debug!("Insert user success");

        tracing::debug!("Retrieving user info from DB");
        let user = sqlx::query_as::<_, User>("SELECT * FROM user_ WHERE email = $1")
            .bind(&user_dto.email)
            .fetch_one(self.inner())
            .await?;
        tracing::debug!("Retrieval success");

        Ok(user)
    }

    pub async fn get_user(&self, user_id: &Uuid) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as::<_, User>(
            r#"
            SELECT * FROM user_ WHERE id = $1
        "#,
        )
        .bind(user_id)
        .fetch_one(self.inner())
        .await?;

        Ok(user)
    }

    pub async fn signin_user(
        &self,
        user_info: &LoginUserDto,
        jwt_secret: &Secret<String>,
    ) -> Result<String, UserError> {
        let query_result = sqlx::query_as::<_, User>("SELECT * FROM user_ WHERE email = $1")
            .bind(&user_info.email)
            .fetch_optional(self.inner())
            .await?;

        let user = query_result.as_ref().ok_or(UserError::UserNotFound)?;

        let hash = PasswordHash::new(&user.password).map_err(UserError::PasswordHashError)?;

        Argon2::default()
            .verify_password(user_info.password.expose_secret().as_bytes(), &hash)
            .map_err(UserError::InvalidCredentials)?;

        let now = Utc::now();
        let iat = now.timestamp() as usize;
        let exp = (now + Duration::minutes(60)).timestamp() as usize;
        let claims: TokenClaims = TokenClaims {
            sub: user.id.to_string(),
            exp,
            iat,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(jwt_secret.expose_secret().as_ref()),
        )?;

        Ok(token)
    }
}

#[derive(Debug, Error)]
pub enum UserError {
    #[error("Failed to hash password")]
    PasswordHashError(password_hash::Error),
    #[error("Error when persisting user: {0}")]
    SqlxError(#[from] sqlx::Error),
    #[error("User not found")]
    UserNotFound,
    #[error("Password or email address is invalid")]
    InvalidCredentials(password_hash::Error),
    #[error("Failed to encode the jwt")]
    JwtError(#[from] jsonwebtoken::errors::Error),
}
