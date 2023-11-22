//! Responsible for all endpoints that require authentication.

use crate::{database::Database, middleware::auth::validator};
use actix_web::{dev::ServiceRequest, web};
use actix_web_httpauth::{extractors::basic::BasicAuth, middleware::HttpAuthentication};

mod my_user;

pub fn private_services(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            // .wrap(HttpAuthentication::bearer(validator))
            .wrap(HttpAuthentication::basic(process_basic))
            .route("/my_user", web::get().to(my_user::my_user)),
    );
}

async fn process_basic(
    mut req: ServiceRequest,
    credentials: BasicAuth,
) -> Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {
    let db = req.extract::<web::Data<Database>>().await.unwrap();

    dbg!(&db);
    todo!()
}
