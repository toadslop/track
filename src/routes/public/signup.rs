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

    match user::actions::signup(&db, user_data.into_inner()).await {
        Ok(user) => {
            tracing::info!("Signup success: {user:?}");

            Ok(HttpResponse::Ok()
                .json(serde_json::json!({"message": "Account successfully created", "user": user})))
        }
        Err(e) => {
            tracing::error!("Failed to persist user: {e}");
            return Err(e);
        }
    }
}

impl ResponseError for SignupError {
    fn status_code(&self) -> StatusCode {
        match self {
            SignupError::InvalidPayload => StatusCode::BAD_REQUEST,
            SignupError::Validation { .. } => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
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
        let cause = match value {
            SignupError::InvalidPayload => Some("required user_id and password".into()),
            SignupError::UserAlreadyExists(..) => Some("already same user_id is used".into()),
            SignupError::Validation { field, reason } => {
                Some(format!("Submission for field {field} is invalid: {reason}"))
            }
            _ => ErrorResponse::default().cause,
        };

        Self {
            cause,
            message: "Account creation failed".into(),
        }
    }
}
