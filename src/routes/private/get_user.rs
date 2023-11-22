use crate::database::Database;
use crate::domain::user;
use crate::error::ErrorResponse;
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, ResponseError};
use uuid::Uuid;

#[tracing::instrument]
pub async fn get_user(
    db: web::Data<Database>,
    user_id: web::Path<String>,
) -> Result<HttpResponse, user::actions::GetOneError> {
    tracing::info!("User info requested for user: {:?}", &user_id.as_ref());

    match user::actions::get_one_by_str_id(&db, &user_id).await {
        Ok(user) => {
            tracing::info!("Request success: {user:?}");
            Ok(HttpResponse::Ok()
                .json(serde_json::json!({"message": "User details by user_id", "user": user})))
        }
        Err(e) => {
            tracing::error!("Request failure: {e}");
            return Err(e);
        }
    }
}

// impl ResponseError for user::actions::GetOneError {
//     fn status_code(&self) -> StatusCode {
//         match self {
//             user::actions::GetOneError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
//             user::actions::GetOneError::NotFound(_) => StatusCode::BAD_REQUEST,
//         }
//     }

//     fn error_response(&self) -> HttpResponse {
//         let response: ErrorResponse = self.into();
//         HttpResponse::build(self.status_code())
//             .content_type("application/json")
//             .json(response)
//     }
// }

// impl From<&user::actions::GetOneError> for ErrorResponse
// where
//     user::actions::GetOneError: ResponseError,
// {
//     fn from(value: &user::actions::GetOneError) -> Self {
//         let cause = match value {
//             user::actions::GetOneError::DatabaseError(_) => ErrorResponse::default().message,
//             user::actions::GetOneError::NotFound(_) => {
//                 "No data was found for the requested user".into()
//             }
//         };

//         Self {
//             cause,
//             message: "Failed to retrieve user information".into(),
//         }
//     }
// }
