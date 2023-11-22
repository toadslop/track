//! Responsible for all endpoints that don't require authentication.

use actix_web::web;
mod health;
mod signin;
mod signup;

pub fn public_services(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .route("/health_check", web::get().to(health::health_check))
            .route("/signup", web::post().to(signup::signup))
            .route("/signin", web::post().to(signin::signin)),
    );
}
