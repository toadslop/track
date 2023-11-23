use thiserror::Error;

use crate::{
    database::Database,
    domain::user::{
        actions::{get_one::UserIdType, GetOneError},
        dto::{GetUserResponse, UpdateUserDto},
        User,
    },
};

use super::signup::UserId;

/// Action for retrieving a single user by it's ID.
#[tracing::instrument]
pub async fn update_user(
    db: &Database,
    user_id: &str,
    update_user: &UpdateUserDto,
) -> Result<GetUserResponse, UpdateError> {
    tracing::debug!("Updating user: {:?}", update_user);

    let update_user: ValidUpdate = update_user.try_into()?;
    let nickname = update_user
        .nickname
        .as_ref()
        .map(|nickname| Some(nickname.as_ref()));

    let comment = update_user
        .comment
        .as_ref()
        .map(|comment| Some(comment.as_ref()));

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
    .bind(nickname)
    .bind(comment)
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
    #[error("Value for field '{field}' is invalid: '{reason}'")]
    Validation { field: String, reason: String },
}

type Nickname = UserId;

#[derive(Debug)]
pub struct ValidUpdate {
    comment: Option<Comment>,
    nickname: Option<Nickname>,
}

impl TryFrom<&UpdateUserDto> for ValidUpdate {
    type Error = UpdateError;

    fn try_from(value: &UpdateUserDto) -> Result<Self, Self::Error> {
        let comment: Option<Comment> = match &value.comment {
            Some(id) => Some(id.to_owned().try_into()?),
            None => None,
        };

        let nickname = match &value.nickname {
            Some(nickname) => {
                Some(
                    nickname
                        .to_owned()
                        .try_into()
                        .map_err(|_| UpdateError::Validation {
                            field: "comment".into(),
                            reason: format!(
                                "Field must be less than or equal to{}",
                                Comment::MAX_LENGTH
                            ),
                        })?,
                )
            }
            None => None,
        };

        Ok(Self { comment, nickname })
    }
}

#[derive(Debug)]
pub struct Comment(String);

impl Comment {
    const MAX_LENGTH: usize = 30;
}

impl TryFrom<String> for Comment {
    type Error = UpdateError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() >= Self::MAX_LENGTH {
            return Err(UpdateError::Validation {
                field: "comment".into(),
                reason: format!("must be less or equal to {}", Self::MAX_LENGTH),
            });
        }

        Ok(Self(value))
    }
}

impl AsRef<str> for Comment {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}
