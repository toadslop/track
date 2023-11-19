use actix_web::http::StatusCode;
use serde::Serialize;

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
