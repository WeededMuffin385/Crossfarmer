pub struct FightRequest {
	pub token: Uuid,
	pub creature_id: usize,
}

pub struct FightResponse {
	pub creature_id: usize,
	pub damage_dealt: usize,
	pub damage_taken: usize,
}