mod accounts;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use actix_web::middleware::Logger;
use log::info;
use crate::accounts::create_accounts;


async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    info!("starting HTTP server at http://localhost:8080");

    let accounts = create_accounts();

    HttpServer::new(move || {
        App::new()
            .app_data(accounts.clone())
            .route("/", web::route().to(hello))
            .configure(accounts::config)
            .wrap(Logger::default())
            .wrap(Cors::permissive())
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}