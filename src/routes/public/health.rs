use actix_web::{get, HttpResponse, Responder};

// TODO: convert to same as signup function
#[get("/health_check")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}
