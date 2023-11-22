use super::User;
use secrecy::Secret;
use serde::{Deserialize, Serialize};

/// User submitted data for signing up
#[derive(Debug, Deserialize)]
pub struct Signup {
    pub user_id: String,
    pub password: Secret<String>,
}

/// User submitted data for signing up
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

/// User submitted data used for signing in.
#[derive(Debug, Deserialize)]
pub struct Signin {
    pub user_id: String,
    pub password: Secret<String>,
}
