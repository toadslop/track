use actix_web::{dev::Server, web, App, HttpServer};
use database::Database;
use routes::{private::private_services, public::public_services};
use std::net::TcpListener;

pub mod app;
pub mod configuration;
pub mod database;
pub mod domain;
mod routes;
pub mod telemetry;

pub async fn run(listener: TcpListener, db: Database) -> anyhow::Result<Server> {
    let db = web::Data::new(db);
    let server = HttpServer::new(move || {
        App::new()
            .configure(public_services)
            .configure(private_services)
            .app_data(db.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
