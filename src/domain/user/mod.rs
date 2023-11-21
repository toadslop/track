use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod actions;
pub mod dto;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct User {
    pub name: String,
    pub email: String,
    pub password: String,
    pub id: Uuid,
}
