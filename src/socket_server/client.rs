use ws::{Sender,Handshake,Handler,Result,CloseCode,Message};
use socket_server::gamestate;
use std::sync::{Arc,Mutex};
use socket_server::gamestate::GameState;
pub struct Client{
	pub id: u64,
	pub out: Sender,
	gamestate: Arc<Mutex<GameState>>,
}

impl Handler for Client {
	fn on_open(&mut self, _: Handshake) -> Result<()> {
		println!("New socket: {}", self.id);
		self.out.send("Hello precious websocket friend. <3")
	}
	fn on_close(&mut self,code: CloseCode, reason: &str){
		match code {
			CloseCode::Normal => println!("Client {}: Closed Gracefully.", self.id),
			CloseCode::Away => println!("Client {}: Left the site.", self.id),
			_=>println!("Unhandled error: {}", reason),
		}
		let mutex = self.gamestate.clone();
		let mut gs = mutex.lock().unwrap();
		gs.deregister(self.id);
	}
	fn on_message(&mut self, message: Message) -> Result<()>{
		let mutex = self.gamestate.clone();
		let mut gs = mutex.lock().unwrap();
		let text = match message.clone().into_text(){
			Ok(s)=>s,
			_=>panic!("aaaaa"),
		};
		gs.sendAll(text,self.id);
		self.out.send(message.clone())
	}
}

pub fn new(id:u64,out: Sender, gs: Arc<Mutex<GameState>>) -> Client{
	Client{id:id,out:out,gamestate:gs}
}