use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod actions;
pub mod dto;

/// Represents a user as stored in the database.
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct User {
    pub name: String,
    pub email: String,
    pub password: String,
    pub id: Uuid,
}

// TODO: add the rest of the database fields
// TODO: create a DTO for returning user data with requests and use
// it in the actions/routes
