mod entrance;
pub mod gameplay;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
	cfg.service(web::scope("/api")
		.configure(entrance::config)
		.configure(gameplay::config));
}