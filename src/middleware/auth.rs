//! Middleware for extracting a JWT from the Authentication header and validating it.

use crate::auth::TokenClaims;
use crate::configuration::auth::AuthSettings;
use crate::error::ErrorResponse;
use actix_web::dev::ServiceRequest;
use actix_web::http::StatusCode;
use actix_web::{web, HttpMessage};
use actix_web::{HttpResponse, ResponseError};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use jsonwebtoken::{decode, DecodingKey, Validation};
use secrecy::ExposeSecret;
use thiserror::Error;

/// Accepts a [ServiceRequest] and [BearerAuth] and confirms the token is valid.
#[tracing::instrument]
pub async fn validator(
    req: ServiceRequest,
    creds: BearerAuth,
) -> Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {
    tracing::info!("Checking auth");

    let settings = match req.app_data::<web::Data<AuthSettings>>().ok_or_else(|| {
        tracing::error!("{}", AuthError::MissingConfig);
        AuthError::MissingConfig
    }) {
        Ok(settings) => settings,
        Err(e) => return Err((e.into(), req)),
    };

    let token = creds.token();

    // TODO: move decode and validate logic to the auth module
    let claims = match decode::<TokenClaims>(
        token,
        &DecodingKey::from_secret(settings.jwtsecret.expose_secret().as_ref()),
        &Validation::default(),
    )
    .map(|c| c.claims)
    .map_err(|e| {
        tracing::error!("Failed to decode auth token: {e}");
        AuthError::InvalidToken(e)
    }) {
        Ok(claims) => claims,
        Err(e) => return Err((e.into(), req)),
    };

    let user_id = uuid::Uuid::parse_str(claims.sub.as_str()).unwrap();
    req.extensions_mut()
        .insert::<uuid::Uuid>(user_id.to_owned());

    Ok(req)
}

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Failed to load authentication configuration")]
    MissingConfig,
    #[error("Token is invalid")]
    InvalidToken(#[from] jsonwebtoken::errors::Error),
}

impl ResponseError for AuthError {
    fn status_code(&self) -> StatusCode {
        match self {
            AuthError::MissingConfig => StatusCode::INTERNAL_SERVER_ERROR,
            AuthError::InvalidToken(_) => StatusCode::UNAUTHORIZED,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let response: ErrorResponse = self.into();
        HttpResponse::build(self.status_code())
            .content_type("application/json")
            .json(response)
    }
}

impl From<&AuthError> for ErrorResponse
where
    AuthError: ResponseError,
{
    fn from(value: &AuthError) -> Self {
        match value {
            AuthError::MissingConfig => ErrorResponse::default(),
            AuthError::InvalidToken(..) => Self {
                cause: value.to_string(),
                message: "Authentication failures".into(),
            },
        }
    }
}
