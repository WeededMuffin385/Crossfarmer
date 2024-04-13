use rusqlite::Row;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct AttackRequest {
	pub id: usize,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Creature {
	pub id: usize,
	pub name: String,
	pub level: usize,
	pub health: usize,
}