use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod actions;
pub mod dto;

/// Represents a user as stored in the database.
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct User {
    pub user_id: String,
    pub password: String,
    pub id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[sqlx(default)]
    pub nickname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[sqlx(default)]
    pub comment: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct BasicId(String);

impl BasicId {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for BasicId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for BasicId {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

impl From<BasicId> for String {
    fn from(value: BasicId) -> Self {
        value.0
    }
}

impl From<&BasicId> for String {
    fn from(value: &BasicId) -> Self {
        value.0.clone()
    }
}

// TODO: add domain validations to email/password/etc
// TODO: add the rest of the database fields
// TODO: create a DTO for returning user data with requests and use
// it in the actions/routes
