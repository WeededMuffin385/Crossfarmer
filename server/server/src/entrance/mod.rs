use std::sync::{Mutex};
use actix_web::{HttpResponse, post, Responder, web};
use serde::{Deserialize, Serialize};
use crate::database::accounts::{account_create, account_exists, account_password};
use crate::database::Pool;
use crate::database::sessions::session_create;

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
struct RecoveryRequest {
    mail: String,
}

#[derive(Serialize, Deserialize)]
struct AuthorizationResponse {
    token: String,
}

#[derive(Serialize, Deserialize)]
struct BadRequestResponse {
    error: String,
}

#[post("/authorization")]
async fn authorization(authorization_data: web::Json<AuthorizationRequest>, pool: web::Data<Pool>) -> impl Responder {
    let AuthorizationRequest {mail, password} = authorization_data.into_inner();

    if account_exists(&pool, &mail) {
        if account_password(&pool, &mail, &password) {
            let token = session_create(&pool, &mail);

            HttpResponse::Ok().json(AuthorizationResponse{
                token: token.to_string(),
            })
        } else {
            HttpResponse::BadRequest().json(BadRequestResponse {
                error: "Wrong password".to_string(),
            })
        }
    } else {
        HttpResponse::BadRequest().json(BadRequestResponse{
            error: "Account doesn't exist".to_string(),
        })
    }
}

#[post("/registration")]
async fn registration(registration_data: web::Json<RegistrationRequest>, pool: web::Data<Pool>) -> impl Responder {
    let RegistrationRequest { mail, username, password } = registration_data.into_inner();

    if account_exists(&pool, &mail) {
        return HttpResponse::BadRequest().json(BadRequestResponse {
            error: "Account already exists".to_string(),
        });
    } else {
        account_create(&pool, &mail, &username, &password);
        return HttpResponse::Ok().finish();
    }
}

#[post("/recovery")]
async fn recovery(recovery_data: web::Json<RecoveryRequest>, pool: web::Data<Pool>) -> impl Responder {
    let RecoveryRequest{mail} = recovery_data.into_inner();

    if account_exists(&pool, &mail) {
        return HttpResponse::Ok().finish();
    } else {
        return HttpResponse::BadRequest().json(BadRequestResponse {
            error: "Account doesn't exist".to_string(),
        });
    }
}


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(authorization).service(registration).service(recovery);
}