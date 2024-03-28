use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RegistrationRequest {
	pub mail: String,
	pub username: String,
	pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct AuthorizationRequest {
	pub mail: String,
	pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct RecoveryRequest {
	pub mail: String,
}

#[derive(Serialize, Deserialize)]
pub struct AuthorizationResponse {
	pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct BadRequestResponse {
	pub error: String,
}