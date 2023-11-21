use crate::database::Database;
use crate::domain::user::actions::SignupError;
use crate::domain::user::{self};
use crate::error::ErrorResponse;
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, ResponseError};

#[tracing::instrument]
pub async fn signup(
    user_data: web::Json<user::dto::Signup>,
    db: web::Data<Database>,
) -> Result<HttpResponse, SignupError> {
    tracing::info!("Signup requested: {user_data:?}");

    match user::actions::signup(&db, &user_data.into_inner()).await {
        Ok(user) => {
            tracing::info!("Signup success: {user:?}");
            Ok(HttpResponse::Ok().json(user))
        }
        Err(e) => {
            tracing::error!("Failed to persist user: {e}");
            return Err(e);
        }
    }
}

impl ResponseError for SignupError {
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
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
        Self {
            status_code: value.status_code().as_u16(),
            message: ErrorResponse::default().message,
        }
    }
}
