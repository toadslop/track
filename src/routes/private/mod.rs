//! Responsible for all endpoints that require authentication.

use crate::middleware::auth::process_basic;
use actix_web::web::{self};

use actix_web_httpauth::middleware::HttpAuthentication;

mod close_account;
mod get_user;
mod my_user;
mod patch_user;

pub fn private_services(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            // .wrap(HttpAuthentication::bearer(validator))
            .wrap(HttpAuthentication::with_fn(process_basic))
            .route("/{user_id}", web::get().to(get_user::get_user))
            .route("/{user_id}", web::patch().to(patch_user::patch_user))
            .route("/my_user", web::get().to(my_user::my_user)),
    )
    .service(
        web::scope("/close").route(
            "",
            web::post()
                .to(close_account::close_account)
                .wrap(HttpAuthentication::with_fn(process_basic)),
        ),
    );
}
