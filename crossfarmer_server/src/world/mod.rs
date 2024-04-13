use std::time::Instant;
use actix_web::web::Data;
use tokio::sync::oneshot;
use crate::database::Pool;

pub async fn run(pool: Data<Pool>) {
	let mut instant = Instant::now();

	loop {
		if instant.elapsed().as_secs_f32() >= 1.0 {
			let creatures = crate::database::creatures::list(&pool);
			if creatures.len() < 30 { crate::database::creatures::spawn(&pool); }
			println!("World is alive. Creatures: {}", creatures.len());


			instant = Instant::now();
		}

	}
}