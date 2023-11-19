use actix_web::web;

// TODO: wrap in auth
pub fn private_services(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/private"));
}
