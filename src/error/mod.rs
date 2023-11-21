//! Contains error types that are used in multiple modules throughout the application.

use actix_web::http::StatusCode;
use serde::Serialize;

/// A standard error response format for consistent error formatting throughout
/// the application. It defaults to an internal server error.
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub status_code: u16,
    pub message: String,
}

impl Default for ErrorResponse {
    fn default() -> Self {
        Self {
            status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            message: "An internal server error occurred".into(),
        }
    }
}
