use actix_web::HttpResponse;

// TODO: convert to same as signup function
#[tracing::instrument]
pub async fn health_check() -> HttpResponse {
    tracing::info!("Health check requested");
    HttpResponse::Ok().finish()
}
