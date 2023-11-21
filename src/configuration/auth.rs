use secrecy::Secret;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AuthSettings {
    pub jwtsecret: Secret<String>,
    pub jwt_expires_in: String,
    pub jwt_max_age: i32,
}

impl Default for AuthSettings {
    fn default() -> Self {
        Self {
            jwtsecret: Secret::new("super_secret".into()), // This is never used
            jwt_expires_in: "60m".into(),
            jwt_max_age: 60,
        }
    }
}
