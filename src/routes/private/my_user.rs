use crate::database::Database;
use crate::routes::error::ErrorResponse;
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, ResponseError};
use thiserror::Error;
use uuid::Uuid;

#[tracing::instrument]
pub async fn my_user(
    db: web::Data<Database>,
    user_id: web::ReqData<Uuid>,
) -> Result<HttpResponse, GetUserError> {
    // tracing::info!("Signup requested: {user_data:?}");

    match db.get_user(&user_id.into_inner()).await {
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
pub enum GetUserError {
    #[error("Failed to persist the user: {0}")]
    PersistanceError(#[from] sqlx::Error),
}

impl ResponseError for GetUserError {
    fn status_code(&self) -> StatusCode {
        match self {
            GetUserError::PersistanceError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let response: ErrorResponse = self.into();
        HttpResponse::build(self.status_code())
            .content_type("application/json")
            .json(response)
    }
}

impl From<&GetUserError> for ErrorResponse
where
    GetUserError: ResponseError,
{
    fn from(value: &GetUserError) -> Self {
        match value {
            GetUserError::PersistanceError(_) => Self {
                status_code: value.status_code().as_u16(),
                message: value.to_string(),
            },
        }
    }
}
