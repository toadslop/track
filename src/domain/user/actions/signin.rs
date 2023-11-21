use crate::{
    auth::{issue_jwt, verify_jwt, JwtError},
    database::Database,
    domain::user::{dto, User},
};
use secrecy::Secret;
use thiserror::Error;

/// Carries out the necessary procedures needed to authenticate a user. It
/// returns a valid JWT.
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

    verify_jwt(&user.password, &user_info.password)?;

    let token = issue_jwt(&user.id, jwt_secret)?;

    Ok(token)
}

#[derive(Debug, Error)]
pub enum SigninError {
    #[error("No user with the provided password and email combination was found")]
    UserNotFound,
    #[error("Error when persisting user: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Error occurred when preparing JWT: {0}")]
    JwtError(#[from] JwtError),
}
