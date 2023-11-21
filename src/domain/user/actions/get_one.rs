use crate::{database::Database, domain::user::User};
use thiserror::Error;
use uuid::Uuid;

#[tracing::instrument]
pub async fn get_one(db: &Database, user_id: &Uuid) -> Result<User, GetOneError> {
    tracing::debug!("Requesting user from db");
    let user = sqlx::query_as::<_, User>(
        r#"
        SELECT * FROM user_ WHERE id = $1
    "#,
    )
    .bind(user_id)
    .fetch_optional(db.inner())
    .await?
    .ok_or(GetOneError::NotFound(*user_id))?;

    tracing::debug!("User found");

    Ok(user)
}

#[derive(Debug, Error)]
pub enum GetOneError {
    #[error("An error occurred with the database when requesting a single user: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("A user with the id '{0}' was not found")]
    NotFound(Uuid),
}
