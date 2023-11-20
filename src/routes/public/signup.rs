use crate::database::Database;
use crate::domain::user::CreateUserDto;
use crate::routes::error::ErrorResponse;
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, ResponseError};
use thiserror::Error;

#[tracing::instrument]
pub async fn signup(
    user_data: web::Json<CreateUserDto>,
    db: web::Data<Database>,
) -> Result<HttpResponse, SignupError> {
    tracing::info!("Signup requested: {user_data:?}");

    match db.insert_user(&user_data.into_inner()).await {
        Ok(user) => {
            tracing::info!("Signup success: {user:?}");
            Ok(HttpResponse::Ok().json(user))
        }
        Err(e) => {
            tracing::error!("Failed to persist user: {e}");
            return Err(e.into());
        }
    }
}

#[derive(Debug, Error)]
pub enum SignupError {
    #[error("Failed to persist the user: {0}")]
    DatabaseError(#[from] sqlx::Error),
}

impl ResponseError for SignupError {
    fn status_code(&self) -> StatusCode {
        match self {
            SignupError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
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
        match value {
            SignupError::DatabaseError(_) => Self {
                status_code: value.status_code().as_u16(),
                message: value.to_string(),
            },
        }
    }
}
