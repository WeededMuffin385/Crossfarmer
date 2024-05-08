use std::time::Instant;
use actix_web::rt::signal;
use actix_web::web::Data;
use tokio::sync::oneshot;
use crate::database::Pool;

pub fn run(pool: Data<Pool>) {
	let mut instant = Instant::now();

	let mut rt = tokio::runtime::Runtime::new().expect("unable to create runtime");
	let (sender, mut receiver) = oneshot::channel::<bool>();

	rt.spawn(async move {
		signal::ctrl_c().await.expect("Failed to listen for ctrl_c");
		sender.send(true).expect("Unable to send break");
		println!("World is stopping due to Ctrl+C signal");
	});

	loop {
		if instant.elapsed().as_secs_f32() >= 1.0 {
			let conn = pool.get().unwrap();
			let creatures = crate::database::creatures::list(&conn);
			if creatures.len() < 30 { crate::database::creatures::spawn(&conn); }
			println!("World is alive. Creatures: {}", creatures.len());
			instant = Instant::now();
		}

		if let Ok(_) = receiver.try_recv() {
			println!("World stopped");
			break;
		}
	}

	println!("World shut down");
}