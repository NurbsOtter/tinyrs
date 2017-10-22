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
	pub fn deregister(&mut self, id: u64){
		self.clients.remove(&id);
	}
	pub fn sendAll(&mut self, message: String,id: u64){
		for (key,_) in &self.clients{
			if *key != id{
				match self.clients.get(&key){
					Some(c)=>{c.send(message.clone());},
					None=>println!("aaaa"),
				};
			}
		}
	}
}