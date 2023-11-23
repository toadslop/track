use crate::database::Database;
use crate::domain::user::{self, BasicId};
use crate::error::ErrorResponse;
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, ResponseError};

#[tracing::instrument]
pub async fn close_account(
    db: web::Data<Database>,
    requester_id: web::ReqData<BasicId>,
) -> Result<HttpResponse, user::actions::DeleteError> {
    tracing::info!("Requested to delete user {}", requester_id.as_str());
    match user::actions::delete(&db, &requester_id).await {
        Ok(user) => {
            tracing::info!("Request success: {user:?} deleted");

            Ok(HttpResponse::Ok()
                .json(serde_json::json!({"message": "Account and user successfully removed"})))
        }
        Err(e) => {
            tracing::error!("Request failure: {e}");
            return Err(e);
        }
    }
}

impl ResponseError for user::actions::DeleteError {
    fn status_code(&self) -> StatusCode {
        match self {
            user::actions::DeleteError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            user::actions::DeleteError::NotFound(_) => todo!(),
        }
    }

    fn error_response(&self) -> HttpResponse {
        let response: ErrorResponse = self.into();
        HttpResponse::build(self.status_code())
            .content_type("application/json")
            .json(response)
    }
}

impl From<&user::actions::DeleteError> for ErrorResponse
where
    user::actions::DeleteError: ResponseError,
{
    fn from(value: &user::actions::DeleteError) -> Self {
        let cause = match value {
            user::actions::DeleteError::DatabaseError(_) => Some(ErrorResponse::default().message),
            user::actions::DeleteError::NotFound(_) => todo!(),
        };

        Self {
            cause,
            message: "Failed to delete the account".into(),
        }
    }
}
