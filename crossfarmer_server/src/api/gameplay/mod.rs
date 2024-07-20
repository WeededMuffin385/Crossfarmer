use actix_web::{get, Responder, web};
use crate::api::entrance::{authorization, recovery, registration};
use crate::database::Pool;

pub mod creatures;
mod accounts;

pub fn config(cfg: &mut web::ServiceConfig) {
	cfg.service(web::scope("/gameplay").configure(creatures::config).configure(accounts::config));
}