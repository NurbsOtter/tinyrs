use socket_server::client::Client;
use ws::Sender;
use std::collections::HashMap;
pub struct GameState{
	clients: HashMap<u64,Sender>,
}

pub fn new()->GameState{
	GameState{clients: HashMap::new()}
}

impl GameState {
	pub fn register(&mut self,sender: Sender,id: u64){
		self.clients.insert(id,sender);
	}
	pub fn sendAll(&mut self, message: String){
		for (key,_) in &self.clients{
			match self.clients.get(&key){
				Some(c)=>{c.send(message.clone());},
				None=>println!("aaaa"),
			};
		}
	}
}