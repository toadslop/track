use actix_web::web;
mod health;
mod signup;

pub fn public_services(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/public")
            .service(health::health_check)
            .route("/signup", web::post().to(signup::signup)),
    );
}
