use crate::configuration::auth::AuthSettings;
use crate::database::Database;
use crate::domain::user::{self};
use crate::error::ErrorResponse;
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, ResponseError};
use serde_json::json;

#[tracing::instrument]
pub async fn signin(
    user_data: web::Json<user::dto::Signin>,
    settings: web::Data<AuthSettings>,
    db: web::Data<Database>,
) -> Result<HttpResponse, user::actions::SigninError> {
    tracing::info!("Signin requested: {user_data:?}");

    match user::actions::signin(&db, &user_data.into_inner(), &settings.jwtsecret).await {
        Ok(jwt) => {
            tracing::info!("Signin success: {jwt:?}");
            Ok(HttpResponse::Ok().json(json!({"token": jwt})))
        }
        Err(e) => {
            tracing::error!("Signin Failure: {e}");
            return Err(e);
        }
    }
}

impl ResponseError for user::actions::SigninError {
    fn status_code(&self) -> StatusCode {
        match self {
            user::actions::SigninError::UserNotFound => StatusCode::BAD_REQUEST,
            user::actions::SigninError::JwtError(source) => match source {
                crate::auth::JwtError::PasswordHash(_) => StatusCode::INTERNAL_SERVER_ERROR,
                crate::auth::JwtError::InvalidCredentials(_) => StatusCode::UNAUTHORIZED,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            },
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

impl From<&user::actions::SigninError> for ErrorResponse
where
    user::actions::SigninError: ResponseError,
{
    fn from(value: &user::actions::SigninError) -> Self {
        let message = match value {
            user::actions::SigninError::UserNotFound => {
                "User for submitted credentials does not exist".into()
            }
            user::actions::SigninError::JwtError(crate::auth::JwtError::InvalidCredentials(_)) => {
                "The username and/or password submitted to not match any user in the system".into()
            }
            _ => "An internal server error occurred".into(),
        };

        Self {
            status_code: value.status_code().as_u16(),
            message,
        }
    }
}
