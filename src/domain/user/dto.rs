use secrecy::Secret;
use serde::Deserialize;

/// User submitted data for signing up
#[derive(Debug, Deserialize)]
pub struct Signup {
    pub user_id: String,
    pub password: Secret<String>,
}

/// User submitted data used for signing in.
#[derive(Debug, Deserialize)]
pub struct Signin {
    pub user_id: String,
    pub password: Secret<String>,
}
