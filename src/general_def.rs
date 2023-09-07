use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    net::TcpStream,
    sync::{Arc, Mutex, RwLock},
};

pub trait ArcMutex {
    fn arc_mutex(&self) -> Arc<Mutex<Self>>;
}
impl ArcMutex for TcpStream {
    fn arc_mutex(&self) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(self.try_clone().unwrap()))
    }
}
pub trait ArcRwLock {
    fn arc_rwlock(&self) -> Arc<RwLock<Self>>;
}
impl ArcRwLock for TcpStream {
    fn arc_rwlock(&self) -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(self.try_clone().unwrap()))
    }
}

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

#[derive(Debug, Clone)]
pub struct Client {
    pub stream_id: String,
    pub stream: Arc<Mutex<TcpStream>>,
}
#[derive(Debug, Clone)]
pub struct Message {
    pub sender: String,
    pub destination: Destinations,
    pub payload: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Destinations {
    Server,
    Private,
    Broadcast,
    Rooms,
}

#[derive(Debug, Clone)]
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
