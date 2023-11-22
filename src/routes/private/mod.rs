//! Responsible for all endpoints that require authentication.

use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;

use crate::middleware::auth::process_basic;

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
