mod messages;

use actix_web::{HttpResponse, post, Responder, web};
use messages::*;
use crate::database::*;

#[post("/authorization")]
async fn authorization(authorization_data: web::Json<AuthorizationRequest>, pool: web::Data<Pool>) -> impl Responder {
	let conn = pool.get().unwrap();
	let AuthorizationRequest {mail, password} = authorization_data.into_inner();

	if accounts::exists(&conn, &mail) {
		if accounts::authorize(&conn, &mail, &password) {
			let token = sessions::create(&conn, &mail);

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
	let conn = pool.get().unwrap();
	let RegistrationRequest { mail, username, password } = registration_data.into_inner();

	if accounts::exists(&conn, &mail) {
		HttpResponse::BadRequest().json(BadRequestResponse {
			error: "Account already exists".to_string(),
		})
	} else {
		accounts::create(&conn, &mail, &username, &password);
		HttpResponse::Ok().finish()
	}
}

#[post("/recovery")]
async fn recovery(recovery_data: web::Json<RecoveryRequest>, pool: web::Data<Pool>) -> impl Responder {
	let conn = pool.get().unwrap();
	let RecoveryRequest{mail} = recovery_data.into_inner();

	if accounts::exists(&conn, &mail) {
		HttpResponse::Ok().finish()
	} else {
		HttpResponse::BadRequest().json(BadRequestResponse {
			error: "Account doesn't exist".to_string(),
		})
	}
}

pub fn config(cfg: &mut web::ServiceConfig) {
	cfg.service(
		web::scope("/entrance")
			.service(authorization)
			.service(registration)
			.service(recovery));
}