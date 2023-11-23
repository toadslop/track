//! Middleware for extracting a JWT from the Authentication header and validating it.

use crate::auth::{verify_password, TokenClaims};
use crate::configuration::auth::AuthSettings;
use crate::database::Database;
use crate::domain::user::User;
use crate::error::ErrorResponse;
use actix_web::dev::ServiceRequest;
use actix_web::http::StatusCode;
use actix_web::{web, HttpMessage};
use actix_web::{HttpResponse, ResponseError};
use actix_web_httpauth::extractors::basic::BasicAuth;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use jsonwebtoken::{decode, DecodingKey, Validation};
use secrecy::{ExposeSecret, Secret};
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
    #[error("Invalid credentials provided")]
    InvalidCredentials,
    #[error("Encountered an error in the database: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Failed to extract the database")]
    DatabaseExtractFailure(#[from] actix_web::Error),
}

impl ResponseError for AuthError {
    fn status_code(&self) -> StatusCode {
        match self {
            AuthError::InvalidToken(_) => StatusCode::UNAUTHORIZED,
            AuthError::InvalidCredentials => StatusCode::UNAUTHORIZED,
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

impl From<&AuthError> for ErrorResponse
where
    AuthError: ResponseError,
{
    fn from(value: &AuthError) -> Self {
        match value {
            AuthError::InvalidToken(..) => Self {
                cause: Some(value.to_string()),
                message: "Authentication Failed".into(),
            },
            AuthError::InvalidCredentials => Self {
                cause: None,
                message: "Authentication Failed".into(),
            },
            _ => Self {
                cause: None,
                message: Self::default().message,
            },
        }
    }
}

#[tracing::instrument]
pub async fn process_basic(
    mut req: ServiceRequest,
    credentials: Option<BasicAuth>,
) -> Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {
    tracing::info!("Requesting signin with basic auth");

    tracing::debug!("Loading database from app data...");
    let db = match req.extract::<web::Data<Database>>().await.map_err(|e| {
        let error: AuthError = AuthError::DatabaseExtractFailure(e);
        tracing::error!("{}", &error);
        error
    }) {
        Ok(db) => db,
        Err(e) => return Err((e.into(), req)),
    };
    tracing::debug!("Database found.");

    tracing::debug!("Extracting credentials...");
    let credentials = match credentials {
        Some(credentials) => credentials,
        None => return Err((AuthError::InvalidCredentials.into(), req)),
    };
    tracing::debug!("Success");

    tracing::debug!("Looking up user data...");
    let user = match sqlx::query_as::<_, User>(
        r#"
        SELECT * FROM user_ WHERE user_id = $1
    "#,
    )
    .bind(credentials.user_id())
    .fetch_optional(db.inner())
    .await
    .map_err(AuthError::DatabaseError)
    {
        Ok(user) => user,
        Err(e) => {
            tracing::error!("Failed to retrieve user info: {e}");
            return Err((e.into(), req));
        }
    };
    tracing::debug!("Request to db succeeded");

    tracing::debug!("Checking if user was found...");
    let user = match user.ok_or(AuthError::InvalidCredentials) {
        Ok(user) => user,
        Err(e) => {
            tracing::error!("User was not found: {e}");
            return Err((e.into(), req));
        }
    };
    tracing::debug!("User found.");

    tracing::debug!("Extracting password from credentials...");
    let submitted_password = match credentials.password() {
        Some(credentials) => credentials,
        None => {
            tracing::error!("Password was not found");
            return Err((AuthError::InvalidCredentials.into(), req));
        }
    }
    .into();
    tracing::debug!("Password found");

    if let Err(e) = verify_password(&user.password, &Secret::new(submitted_password)) {
        tracing::error!("Password verification failed: {e}");
        return Err((AuthError::InvalidCredentials.into(), req));
    }

    Ok(req)
}
