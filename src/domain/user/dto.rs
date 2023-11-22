use super::User;
use secrecy::Secret;
use serde::{Deserialize, Serialize};

/// User submitted data for signing up
#[derive(Debug, Deserialize)]
pub struct Signup {
    pub user_id: String,
    pub password: Secret<String>,
}

/// Response format when user is requested
#[derive(Debug, Serialize)]
pub struct SignupResponse {
    pub user_id: String,
    pub nickname: String,
}

impl From<User> for SignupResponse {
    fn from(value: User) -> Self {
        Self {
            user_id: value.user_id.clone(),
            nickname: value.user_id,
        }
    }
}

/// User submitted data for signing up
#[derive(Debug, Serialize)]
pub struct GetUserResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    pub nickname: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

impl From<User> for GetUserResponse {
    fn from(value: User) -> Self {
        Self {
            user_id: Some(value.user_id.clone()),
            nickname: value.user_id,
            comment: value.comment,
        }
    }
}

/// User submitted data for modifying their account
#[derive(Debug, Deserialize)]
pub struct UpdateUserDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

/// User submitted data used for signing in.
#[derive(Debug, Deserialize)]
pub struct Signin {
    pub user_id: String,
    pub password: Secret<String>,
}
