use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SendRequest {
	pub token: String,
	pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
	pub username: String,
	pub message: String,
}