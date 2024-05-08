use actix_web::{get, HttpResponse, post, Responder, web};
use crate::api::gameplay::creatures::messages::AttackRequest;
use crate::database::Pool;

pub mod messages;

/*
TODO: Player that will fight creature should be recognised via <bearer> token, that is located in <SESSIONS> table
TODO: Idea is to transform token to player, so we would acquire useful data about player stats (like damage)
*/


#[post("/attack")]
async fn attack(attack_data: web::Json<AttackRequest>, pool: web::Data<Pool>) -> impl Responder {
	let conn = pool.get().unwrap();

	let AttackRequest {token, creature_id} = attack_data.into_inner();
	crate::database::creatures::attack(&conn, token, creature_id);
	HttpResponse::Ok().finish()
}


#[get("/list")]
async fn list(pool: web::Data<Pool>) -> impl Responder {
	let conn = pool.get().unwrap();

	let creatures = crate::database::creatures::list(&conn);
	HttpResponse::Ok().json(creatures)
}



pub fn config(cfg: &mut web::ServiceConfig) {
	cfg.service(web::scope("/creatures")
		.service(list)
	 	.service(attack));
}