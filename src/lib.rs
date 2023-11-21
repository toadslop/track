use actix_web::{dev::Server, web, App, HttpServer};
use configuration::auth::AuthSettings;
use database::Database;
use routes::{private::private_services, public::public_services};
use std::net::TcpListener;

pub mod app;
pub mod configuration;
pub mod database;
pub mod domain;
mod middleware;
mod routes;
pub mod telemetry;

pub async fn run(
    listener: TcpListener,
    db: Database,
    auth_settings: AuthSettings,
) -> anyhow::Result<Server> {
    let db = web::Data::new(db);
    let auth_settings = web::Data::new(auth_settings);
    let server = HttpServer::new(move || {
        App::new()
            .configure(public_services)
            .configure(private_services)
            .app_data(db.clone())
            .app_data(auth_settings.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
