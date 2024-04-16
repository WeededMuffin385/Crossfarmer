mod database;
mod api;
mod world;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use actix_web::middleware::Logger;
use actix_web::rt::signal;
use log::info;
use tokio::sync::oneshot;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    info!("starting HTTP server at http://localhost:8080");


    let pool = web::Data::new(database::create());
    let pool_move = pool.clone();
    std::thread::spawn(move||{world::run(pool_move)});

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Cors::permissive())
            .app_data(pool.clone())
            .configure(api::config)
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}