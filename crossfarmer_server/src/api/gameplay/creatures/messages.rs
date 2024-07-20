use rusqlite::Row;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct AttackRequest {
	pub token: String,
	pub creature_id: usize,
}


