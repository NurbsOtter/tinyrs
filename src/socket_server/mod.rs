use ws::listen;
use std::thread;
pub fn new(){
	thread::spawn(move ||{
		let err = listen("127.0.0.1:3012",|out|{
			move |msg|{
				println!("aaaaaa", );
				out.send(msg)
			}
		});
		match err{
			Err(error)=>println!("aaaaaa"),
			Ok(_)=>println!("Listening"),
		}
	});
}