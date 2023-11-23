use crate::domain::user::actions::UpdateError;
use crate::domain::user::{self, BasicId};
use crate::error::ErrorResponse;
use crate::{database::Database, domain::user::dto::UpdateUserDto};
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, ResponseError};

#[tracing::instrument]
pub async fn patch_user(
    db: web::Data<Database>,
    user_id: web::Path<String>,
    update_user: web::Json<UpdateUserDto>,
    requester_id: web::ReqData<BasicId>,
) -> Result<HttpResponse, user::actions::UpdateError> {
    tracing::info!("Request to update user {:?}", &update_user);

    if requester_id.as_str() != user_id.as_str() {
        Err(UpdateError::Forbidden {
            requester: requester_id.as_str().into(),
            requested: user_id.as_str().to_owned(),
        })?
    }

    match user::actions::update_user(&db, &user_id, &update_user).await {
        Ok(user) => {
            tracing::info!("Request success: {user:?}");
            let users = vec![user];
            Ok(HttpResponse::Ok()
                .json(serde_json::json!({"message": "User successfully updated", "recipe": users})))
        }
        Err(e) => {
            tracing::error!("Request failure: {e}");
            return Err(e);
        }
    }
}

impl ResponseError for user::actions::UpdateError {
    fn status_code(&self) -> StatusCode {
        match self {
            user::actions::UpdateError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            user::actions::UpdateError::GetOneError(e) => e.status_code(),
            user::actions::UpdateError::Forbidden { .. } => StatusCode::FORBIDDEN,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let response: ErrorResponse = self.into();
        HttpResponse::build(self.status_code())
            .content_type("application/json")
            .json(response)
    }
}

impl From<&user::actions::UpdateError> for ErrorResponse
where
    user::actions::UpdateError: ResponseError,
{
    fn from(value: &user::actions::UpdateError) -> Self {
        let cause = match value {
            user::actions::UpdateError::DatabaseError(_) => Some(ErrorResponse::default().message),
            user::actions::UpdateError::GetOneError(e) => Some(e.to_string()),
            user::actions::UpdateError::Forbidden { .. } => Some("Unauthorized".into()),
        };

        let message = match value {
            UpdateError::Forbidden { .. } => "No Permission for Update".into(),
            _ => "Failed to retrieve user information".into(),
        };

        Self { cause, message }
    }
}
