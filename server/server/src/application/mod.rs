use actix_web::web;

pub mod entrance;

pub mod database;
mod gameplay;

pub fn config(cfg: &mut web::ServiceConfig) {
	cfg.configure(entrance::config);
}