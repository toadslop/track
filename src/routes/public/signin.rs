use crate::configuration::auth::AuthSettings;
use crate::database::Database;
use crate::domain::user::{LoginUserDto, UserError};
use crate::routes::error::ErrorResponse;
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, ResponseError};
use thiserror::Error;

#[tracing::instrument]
pub async fn signin(
    user_data: web::Json<LoginUserDto>,
    settings: web::Data<AuthSettings>,
    db: web::Data<Database>,
) -> Result<HttpResponse, SignupError> {
    tracing::info!("Signin requested: {user_data:?}");

    match db
        .signin_user(&user_data.into_inner(), &settings.jwtsecret)
        .await
    {
        Ok(jwt) => {
            tracing::info!("Signin success: {jwt:?}");
            Ok(HttpResponse::Ok().json(jwt))
        }
        Err(e) => {
            tracing::error!("Signin Failure: {e}");
            return Err(e.into());
        }
    }
}

#[derive(Debug, Error)]
pub enum SignupError {
    #[error("Failed to persist the user: {0}")]
    PersistanceError(#[from] UserError),
}

impl ResponseError for SignupError {
    fn status_code(&self) -> StatusCode {
        match self {
            SignupError::PersistanceError(source) => match source {
                UserError::PasswordHashError(source) => match source {
                    argon2::password_hash::Error::ParamNameDuplicated => StatusCode::BAD_REQUEST,
                    argon2::password_hash::Error::ParamNameInvalid => StatusCode::BAD_REQUEST,
                    argon2::password_hash::Error::ParamValueInvalid(_) => StatusCode::BAD_REQUEST,
                    argon2::password_hash::Error::ParamsMaxExceeded => StatusCode::BAD_REQUEST,
                    argon2::password_hash::Error::Password => StatusCode::BAD_REQUEST,
                    _ => StatusCode::BAD_REQUEST,
                },
                UserError::SqlxError(source) => match source {
                    sqlx::Error::RowNotFound => StatusCode::BAD_REQUEST,
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                },
                UserError::UserNotFound => StatusCode::BAD_REQUEST,
                UserError::InvalidCredentials(_) => StatusCode::BAD_REQUEST,
                UserError::JwtError(_) => StatusCode::BAD_REQUEST,
            },
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
            SignupError::PersistanceError(_) => Self {
                status_code: value.status_code().as_u16(),
                message: {
                    if value.status_code().as_u16() != 500 {
                        value.to_string()
                    } else {
                        "An internal server error occurred".into()
                    }
                },
            },
        }
    }
}
