mod entrance;
mod database;
mod game;


use actix_cors::Cors;
use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use actix_web::middleware::Logger;
use log::info;
use crate::database::create_database_pool;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    info!("starting HTTP server at http://localhost:8080");

    let pool = create_database_pool();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Cors::permissive())
            .app_data(web::Data::new(pool.clone()))
            .configure(entrance::config)
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}