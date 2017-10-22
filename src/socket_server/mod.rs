use ws::listen;
use std::thread;
use rand;
use rand::Rng;
use std::sync::{Arc, Mutex};
mod client;
mod gamestate;
pub fn new(){
	let gamestate = Arc::new(Mutex::new(gamestate::new()));
	thread::spawn(move ||{
		let mut rng = rand::thread_rng();
		let err = listen("0.0.0.0:3012",|out|{
			let client = client::new(rng.gen::<u64>(), out.clone(),gamestate.clone());
			let mutex = gamestate.clone();
			let mut gs = mutex.lock().unwrap();
			gs.register(out.clone(),client.id);
			return client;
		});
		match err {
			Err(err) =>println!("Socket error"),
			Ok(_)=> println!("Listening"),
		};
	});
}