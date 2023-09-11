use std::{fmt::Display, net::TcpStream, sync::Arc};

use bus::Bus;
use crossbeam_channel::{unbounded, Receiver, Sender};

#[derive(Debug, Clone)]
pub struct CommunicationBus<T> {
    pub sender: Sender<T>,
    pub receiver: Receiver<T>,
}

impl<T> CommunicationBus<T> {
    pub fn new() -> Self {
        let (bus_sender, bus_receiver) = unbounded();
        CommunicationBus {
            sender: bus_sender,
            receiver: bus_receiver,
        }
    }
}

#[derive(Debug, Clone)]
pub enum DynamicValue {
    Client(),
    ClientOFF(),
    ChatMsg(),
    ChatRawMSG(),
}

pub trait RemoteID {
    fn remote_id(&self) -> String;
}
impl RemoteID for TcpStream {
    fn remote_id(&self) -> String {
        self.peer_addr().unwrap().to_string()
    }
}
#[macro_export]
macro_rules! new_thread {
    ($name:expr, $function:expr) => {
        std::thread::Builder::new()
            .name(String::from($name))
            .spawn(move || $function)
            .unwrap();
    };
}

#[macro_export]
macro_rules! pause {
    ($milliseconds:expr) => {
        std::thread::sleep(Duration::from_millis($milliseconds));
    };
}

#[derive(Debug, Clone)]
pub struct BusMessage {
    pub command: String,
    pub sender: String,
    pub destination: String,
    pub payload: String,
}

impl BusMessage {
    pub fn new() -> Self {
        BusMessage {
            command: "".to_string(),
            sender: "".to_string(),
            destination: "".to_string(),
            payload: "".to_string(),
        }
    }

    pub fn msg(&self) -> String {
        format!(
            "{}/{}/{}/{}",
            self.command.to_uppercase(),
            self.sender,
            self.destination.to_uppercase(),
            self.payload
        )
    }
}

#[derive(Debug, Clone)]
pub enum Commands {
    CltConn(String),
    CltDis(String),
    Msg,
}
