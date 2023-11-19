use actix_web::{dev::Server, App, HttpServer};
use routes::{private::private_services, public::public_services};
use std::net::TcpListener;

pub mod app;
pub mod configuration;
pub mod database;
mod routes;
pub mod telemetry;

pub async fn run(listener: TcpListener) -> anyhow::Result<Server> {
    let server = HttpServer::new(|| {
        App::new()
            .configure(public_services)
            .configure(private_services)
    })
    .listen(listener)?
    .run();

    Ok(server)
}
