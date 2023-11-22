use std::fmt::Display;

use crate::{
    database::Database,
    domain::user::{
        actions::get_one,
        dto::{GetUserResponse, UpdateUserDto},
        User,
    },
};
use thiserror::Error;
use uuid::Uuid;

/// Action for retrieving a single user by it's ID.
#[tracing::instrument]
pub async fn update_user(
    db: &Database,
    user_id: &str,
    update_user: &UpdateUserDto,
) -> Result<User, GetOneError> {
    tracing::debug!("Requesting user from db");
    sqlx::query(
        r#"
        UPDATE user_
            SET
                nickname = COALESCE($1, nickname),
                comment = COALESCE($2, comment),
            WHERE user_id = $3;
    "#,
    )
    .bind(&update_user.nickname)
    .bind(&update_user.comment)
    .bind(user_id)
    .fetch_one(db.inner())
    .await?;

    let user = get_one_by_str_id(db, user_id).await?;

    // .await?
    // .ok_or(GetOneError::NotFound(UserIdType::Uuid(*user_id)))?;

    tracing::debug!("User found");
    // dbg!(user);
    todo!();
    // Ok(user)
}

/// Action for retrieving a single user by it's ID.
#[tracing::instrument]
pub async fn get_one_by_str_id(
    db: &Database,
    user_id: &str,
) -> Result<GetUserResponse, GetOneError> {
    tracing::debug!("Requesting user from db");
    let user = sqlx::query_as::<_, User>(
        r#"
        SELECT * FROM user_ WHERE user_id = $1
    "#,
    )
    .bind(user_id)
    .fetch_optional(db.inner())
    .await?
    .ok_or(GetOneError::NotFound(UserIdType::Str(user_id.to_owned())))?;

    tracing::debug!("User found");

    Ok(user.into())
}

#[derive(Debug, Error)]
pub enum GetOneError {
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
