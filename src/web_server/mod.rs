use iron::prelude::*;
use iron;
use mount::Mount;
use std::path::Path;
use logger::Logger;
use env_logger;
use staticfile::Static;
use std::thread;
fn hello_test(_: &mut Request)->IronResult<Response>{
	Ok(Response::with((iron::status::Ok,"Hello world")))
}
pub fn new()->thread::JoinHandle<()>{
	let (logger_before,logger_after) = Logger::new(None);
	let mut static_mount = Mount::new();
	static_mount.mount("/",Static::new(Path::new("client/build/")));
	let mut chain = Chain::new(static_mount);
	chain.link_before(logger_before);
	chain.link_after(logger_after);
	thread::spawn(move ||{
		Iron::new(chain).http("127.0.0.1:3030").unwrap();
	})
}
