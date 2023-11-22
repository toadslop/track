//! Contains error types that are used in multiple modules throughout the application.

use serde::Serialize;

/// A standard error response format for consistent error formatting throughout
/// the application. It defaults to an internal server error.
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub cause: String,
    pub message: String,
}

impl Default for ErrorResponse {
    fn default() -> Self {
        Self {
            cause: "Unknown".into(),
            message: "An internal server error occurred".into(),
        }
    }
}
