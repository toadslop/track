use actix_web::web;
use actix_web::{dev::Server, App, HttpServer};
use routes::{echo, hello, manual_hello};
use std::net::TcpListener;

pub mod app;
pub mod configuration;
mod routes;
pub mod telemetry;

pub async fn run(listener: TcpListener) -> anyhow::Result<Server> {
    let server = HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
