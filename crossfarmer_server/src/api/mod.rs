mod entrance;
pub mod gameplay;
pub(crate) mod messages;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
	cfg.service(web::scope("/api")
		.configure(entrance::config)
		.configure(gameplay::config)
	 	.configure(messages::config));
}