mod entrance;
mod gameplay;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
	cfg.configure(entrance::config);
}