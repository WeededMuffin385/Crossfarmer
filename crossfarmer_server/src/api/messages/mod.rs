pub mod messages;

use actix_web::{get, HttpResponse, post, Responder, web};
use crate::api::messages::messages::SendRequest;
use crate::database::Pool;

#[post("/send")]
async fn send(send_request: web::Json<SendRequest>, pool: web::Data<Pool>) -> impl Responder {
	let conn = pool.get().unwrap();
	let SendRequest {token, message} = send_request.into_inner();

	if let Some(account_id) = crate::database::sessions::get_account_id(&conn, &token) {
		crate::database::messages::send(&pool, account_id, message);
		HttpResponse::Ok()
	} else {
		HttpResponse::BadRequest()
	}
}

#[get("/list")]
async fn list(pool: web::Data<Pool>) -> impl Responder {
	let messages = crate::database::messages::list(&pool);
	HttpResponse::Ok().json(messages)
}

pub fn config(cfg: &mut web::ServiceConfig) {
	cfg.service(web::scope("/messages")
	 .service(send)
	 .service(list));
}