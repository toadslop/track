use crate::middleware::auth::validator;
use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;

mod my_user;

pub fn private_services(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/private")
            .wrap(HttpAuthentication::bearer(validator))
            .route("my_user", web::get().to(my_user::my_user)),
    );
}
