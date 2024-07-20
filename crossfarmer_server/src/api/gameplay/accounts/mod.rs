use actix_web::{HttpResponse, post, Responder, web};
use serde::{Deserialize, Serialize};
use crate::api::gameplay::creatures::{attack, list};
use crate::database::Pool;

#[derive(Serialize, Deserialize, Debug)]
pub struct BalanceRequest {
	pub token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BalanceResponse {
	pub balance: f64,
}

#[post("/balance")]
async fn balance(balance_data: web::Json<BalanceRequest>, pool: web::Data<Pool>) -> impl Responder {

	let conn = pool.get().unwrap();
	let BalanceRequest {token} = balance_data.into_inner();

	let balance = crate::database::accounts::balance(&conn, &token);

	HttpResponse::Ok().json(BalanceResponse{balance})
}

pub fn config(cfg: &mut web::ServiceConfig) {
	cfg.service(web::scope("/accounts")
	 .service(balance));
}