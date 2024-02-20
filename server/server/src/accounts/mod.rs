mod account;

use std::collections::HashMap;
use std::sync::{Mutex};
use actix_web::{HttpResponse, post, Responder, web};
use jsonwebtoken::{Algorithm, decode, DecodingKey, encode, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use crate::accounts::account::Account;

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
    token: String,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    mail: String,
}

const KEY: &[u8] = b"Secret Key_Smartly Made For Me";

fn generate_token(mail: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let encoding_key = EncodingKey::from_secret(KEY);

    let claims = Claims {
        mail: mail.to_string(),
    };

    encode(&Header::default(), &claims, &encoding_key)
}

pub fn validate_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let decoding_key = DecodingKey::from_secret(KEY);

    decode(token, &decoding_key, &Validation::new(Algorithm::HS256)).map(|data| data.claims)
}

#[post("/authorization")]
async fn authorization(authorization_data: web::Json<AuthorizationRequest>, accounts: web::Data<Mutex<Accounts>>) -> impl Responder {
    let AuthorizationRequest {mail, password} = authorization_data.into_inner();
    let accounts = accounts.lock().expect("problem with mutex");

    if let Some(account) = accounts.accounts.get(&mail) {
        if account.password == password {
            let response = AuthorizationResponse {
                token: generate_token(mail.as_str()).expect("unable to create token"),
            };
            let response = serde_json::to_string(&response).expect("unable to serialize bearer message");

            HttpResponse::Ok().body(response)
        } else {
            println!("wrong password: (real){} != {}(input)", account.password, password);
            HttpResponse::BadRequest().finish()
        }
    } else {
        println!("account doesn't exist");
        HttpResponse::BadRequest().finish()
    }
}

#[post("/registration")]
async fn registration(registration_data: web::Json<RegistrationRequest>, accounts: web::Data<Mutex<Accounts>>) -> impl Responder {
    let RegistrationRequest { mail, username, password } = registration_data.into_inner();

    println!("Registered new account. Mail: {}. Username: {}. Password: {}.", &mail, &username, &password);
    let mut accounts = accounts.lock().expect("unable to get value from mutex");
    accounts.accounts.entry(mail.clone()).or_insert(Account::new(mail, username, password));

    HttpResponse::Ok()
}

#[derive(Default)]
pub struct Accounts {
    accounts: HashMap<String, Account>,
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(authorization).service(registration);
}

pub fn create_accounts() -> web::Data<Mutex<Accounts>> {
    web::Data::new(Mutex::new(Accounts::default()))
}