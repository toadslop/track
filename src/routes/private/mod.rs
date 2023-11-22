//! Responsible for all endpoints that require authentication.

use crate::{
    auth::verify_password, database::Database, domain::user::User, middleware::auth::validator,
};
use actix_web::{dev::ServiceRequest, web};
use actix_web_httpauth::{extractors::basic::BasicAuth, middleware::HttpAuthentication};
use secrecy::Secret;

mod get_user;
mod my_user;

pub fn private_services(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            // .wrap(HttpAuthentication::bearer(validator))
            .wrap(HttpAuthentication::basic(process_basic))
            .route("/{user_id}", web::get().to(get_user::get_user))
            .route("/my_user", web::get().to(my_user::my_user)),
    );
}

async fn process_basic(
    mut req: ServiceRequest,
    credentials: BasicAuth,
) -> Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {
    let db = req
        .extract::<web::Data<Database>>()
        .await
        .expect("TODO: handle error correctly");

    let user = sqlx::query_as::<_, User>(
        r#"
        SELECT * FROM user_ WHERE user_id = $1
    "#,
    )
    .bind(credentials.user_id())
    .fetch_optional(db.inner())
    .await
    .expect("TODO")
    .expect("TODO");

    let submitted_password = credentials
        .password()
        .expect("SHOULD HAVE PASSWORD")
        .to_string();

    verify_password(&user.password, &Secret::new(submitted_password)).expect("TODO");

    Ok(req)
}
