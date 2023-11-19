use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

use crate::routes::error::ErrorResponse;

#[tracing::instrument]
pub async fn signup() -> Result<HttpResponse, SignupError> {
    todo!()
}

#[derive(Debug, Error)]
pub enum SignupError {}

impl ResponseError for SignupError {
    fn status_code(&self) -> StatusCode {
        todo!()
    }

    fn error_response(&self) -> HttpResponse {
        let response: ErrorResponse = self.into();
        HttpResponse::build(self.status_code())
            .content_type("application/json")
            .json(response)
    }
}

impl From<&SignupError> for ErrorResponse
where
    SignupError: ResponseError,
{
    fn from(value: &SignupError) -> Self {
        todo!();
    }
}
