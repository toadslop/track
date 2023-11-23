use thiserror::Error;

use crate::{
    database::Database,
    domain::user::{
        actions::{get_one::UserIdType, GetOneError},
        dto::{GetUserResponse, UpdateUserDto},
        User,
    },
};

/// Action for retrieving a single user by it's ID.
#[tracing::instrument]
pub async fn update_user(
    db: &Database,
    user_id: &str,
    update_user: &UpdateUserDto,
) -> Result<GetUserResponse, UpdateError> {
    tracing::debug!("Updating user: {:?}", update_user);
    let user = sqlx::query_as::<_, User>(
        r#"
        UPDATE user_
            SET
                nickname = COALESCE($1, nickname),
                comment = COALESCE($2, comment)
            WHERE user_id = $3
            RETURNING user_id, nickname, comment, password, id;
    "#,
    )
    .bind(&update_user.nickname)
    .bind(&update_user.comment)
    .bind(user_id)
    .fetch_one(db.inner())
    .await?;
    tracing::debug!("Success: {:?}", user);

    let mut user: GetUserResponse = user.into();
    user.user_id = None;
    Ok(user)
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
pub enum UpdateError {
    #[error("An error occurred with the database when requesting a single user: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("An error occurred when retrieving a user from the database: {0}")]
    GetOneError(#[from] GetOneError),
    #[error(
        "A user with the id '{requester}' does not have permission to update user '{requested}'"
    )]
    Forbidden {
        requester: String,
        requested: String,
    },
}
