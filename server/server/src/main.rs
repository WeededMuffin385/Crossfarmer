use actix_web::{middleware, web, App, HttpRequest, HttpServer, Responder, HttpResponse, post, get};
use serde::{Deserialize, Serialize};
use tokio_postgres::NoTls;

#[derive(Serialize, Deserialize)]
struct RegistrationRequest {
    mail: String,
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct AuthorizationRequest {
    mail: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct AuthorizationResponse {
    bearer: String,
}

#[post("/authorization")]
async fn authorization(authorization_data: web::Json<AuthorizationRequest>) -> impl Responder {
    let response = AuthorizationResponse {
        bearer: "token".to_string(),
    };

    let response = serde_json::to_string(&response).expect("unable to serialize bearer message");

    HttpResponse::Ok().body(response)
}

#[post("/registration")]
async fn registration(registration_data: web::Json<RegistrationRequest>) -> impl Responder {
    let token = "token123token";
    HttpResponse::Ok().body(token)
}



#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world")
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::info!("starting HTTP server at http://localhost:8080");


    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}