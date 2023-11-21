use secrecy::Secret;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Signup {
    pub name: String,
    pub email: String,
    pub password: Secret<String>,
}

#[derive(Debug, Deserialize)]
pub struct Signin {
    pub email: String,
    pub password: Secret<String>,
}
