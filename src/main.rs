mod web_server;
mod socket_server;
extern crate iron;
extern crate mount;
extern crate staticfile;
extern crate logger;
extern crate env_logger;
extern crate ws;
fn main() {
	env_logger::init().unwrap();
	let handle = web_server::new();
	socket_server::new();
	println!("Hello, world!");
	handle.join();
    
}
