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
    pub nickname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

// TODO: add domain validations to email/password/etc
// TODO: add the rest of the database fields
// TODO: create a DTO for returning user data with requests and use
// it in the actions/routes
