use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    net::TcpStream,
};

pub trait RemoteID {
    fn remote_id(&self) -> String;
}
impl RemoteID for TcpStream {
    fn remote_id(&self) -> String {
        self.peer_addr().unwrap().to_string()
    }
}

pub trait StringHash {
    fn get_hash(&self) -> String;
}

impl StringHash for str {
    fn get_hash(&self) -> String {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish().to_string()
    }
}

pub trait KeyValuePayload {
    fn value(self) -> DynamicValue;
}

#[derive(Debug)]
pub struct Client {
    pub stream_id: String,
    pub stream: TcpStream,
}
#[derive(Debug)]
pub struct Message {
    pub sender: String,
    pub destination: Destinations,
    pub payload: String,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Destinations {
    Server,
    Private,
    Broadcast,
    Rooms,
}

#[derive(Debug)]
pub enum DynamicValue {
    Client(Client),
    ChatMsg(Message),
}

impl KeyValuePayload for Client {
    fn value(self) -> DynamicValue {
        DynamicValue::Client(self)
    }
}
impl KeyValuePayload for Message {
    fn value(self) -> DynamicValue {
        DynamicValue::ChatMsg(self)
    }
}
