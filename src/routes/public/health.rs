use actix_web::HttpResponse;

// TODO: convert to same as signup function
#[tracing::instrument]
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
