use crate::domain::user;
use crate::{database::Database, domain::user::dto::UpdateUserDto};
use actix_web::{web, HttpResponse};

#[tracing::instrument]
pub async fn patch_user(
    db: web::Data<Database>,
    user_id: web::Path<String>,
    update_user: web::Json<UpdateUserDto>,
) -> Result<HttpResponse, user::actions::GetOneError> {
    tracing::info!("User info requested for user: {:?}", &update_user);

    match user::actions::update_user(&db, &user_id, &update_user).await {
        Ok(user) => {
            tracing::info!("Request success: {user:?}");
            let users = vec![user];
            Ok(HttpResponse::Ok()
                .json(serde_json::json!({"message": "User successfully updated", "recipe": users})))
        }
        Err(e) => {
            tracing::error!("Request failure: {e}");
            todo!();
            // return Err(e);
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
