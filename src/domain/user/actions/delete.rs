use std::fmt::Display;

use crate::{
    database::Database,
    domain::user::{BasicId, User},
};
use thiserror::Error;
use uuid::Uuid;

/// Action for deleting the user.
#[tracing::instrument]
pub async fn delete(db: &Database, user_id: &BasicId) -> Result<User, DeleteError> {
    tracing::debug!("Requesting user from db");
    let user = sqlx::query_as::<_, User>(
        r#"
        DELETE FROM user_ WHERE user_id = $1
        RETURNING *;
    "#,
    )
    .bind(user_id.as_str())
    .fetch_optional(db.inner())
    .await?
    .ok_or(DeleteError::NotFound(UserIdType::Str(String::from(
        user_id,
    ))))?;

    tracing::debug!("User found");

    Ok(user)
}

#[derive(Debug, Error)]
pub enum DeleteError {
    #[error("An error occurred with the database when requesting a single user: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("A user with the id '{0}' was not found")]
    NotFound(UserIdType),
}

#[derive(Debug)]
pub enum UserIdType {
    Uuid(Uuid),
    Str(String),
}

impl Display for UserIdType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserIdType::Uuid(id) => write!(f, "{id}"),
            UserIdType::Str(id) => write!(f, "{id}"),
        }
    }
}
