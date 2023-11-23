use crate::{
    auth::hash_password,
    database::Database,
    domain::user::{
        dto::{self, Signup, SignupResponse},
        User,
    },
};
use argon2::password_hash::{self};
use chrono::Utc;
use secrecy::Secret;
use thiserror::Error;
use uuid::Uuid;

/// Performs the necessary procedures required for signing up a new user.
#[tracing::instrument]
pub async fn signup(db: &Database, user_dto: dto::Signup) -> Result<SignupResponse, SignupError> {
    tracing::debug!("Validating request integrity...");
    let ValidSignup { user_id, password } = user_dto.try_into()?;
    tracing::debug!("Request contains required fields");

    tracing::debug!("Checking if user already exists...");
    let user = sqlx::query_as::<_, User>("SELECT * FROM user_ WHERE user_id = $1")
        .bind(user_id.as_ref())
        .fetch_optional(db.inner())
        .await?;

    if user.is_some() {
        Err(SignupError::UserAlreadyExists(user_id.as_ref().to_owned()))?;
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
    .bind(user_id.as_ref())
    .bind(hashed_password)
    .bind(Utc::now())
    .bind(user_id.as_ref()) // DEFAULT
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
    #[error("Invalid data was submitted: {field} {reason}")]
    Validation { field: String, reason: String },
}

#[derive(Debug)]
pub struct ValidSignup {
    user_id: UserId,
    password: Secret<String>,
}

impl TryFrom<Signup> for ValidSignup {
    type Error = SignupError;

    fn try_from(value: Signup) -> Result<Self, Self::Error> {
        let user_id = match value.user_id {
            Some(id) => id,
            None => Err(SignupError::InvalidPayload)?,
        };

        let user_id: UserId = user_id.try_into()?;

        let password = match value.password {
            Some(password) => password,
            None => Err(SignupError::InvalidPayload)?,
        };

        Ok(Self { user_id, password })
    }
}

#[derive(Debug)]
pub struct UserId(String);

impl UserId {
    const MAX_LENGTH: usize = 20;
    const MIN_LENGTH: usize = 8;
}

impl TryFrom<String> for UserId {
    type Error = SignupError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() >= UserId::MAX_LENGTH {
            return Err(SignupError::Validation {
                field: "user_id".into(),
                reason: format!("must be less or equal to {}", Self::MAX_LENGTH),
            });
        }

        if value.len() < UserId::MIN_LENGTH {
            return Err(SignupError::Validation {
                field: "user_id".into(),
                reason: format!("must be greater than or equal to {}", Self::MAX_LENGTH),
            });
        }

        Ok(Self(value))
    }
}

impl AsRef<str> for UserId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
