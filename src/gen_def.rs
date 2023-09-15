use std::{
    arch::x86_64::_SIDD_NEGATIVE_POLARITY,
    fmt::{self},
    net::TcpStream,
};

use tokio::sync::watch::{
    channel as WATCHChannel, Receiver as WATCHReceiver, Sender as WATCHSender,
};

#[derive(Debug)]
pub struct MessageBus<T> {
    pub sender: WATCHSender<T>,
    pub receiver: WATCHReceiver<T>,
}

#[allow(dead_code)]
impl<T> MessageBus<T> {
    pub fn new(initial_value: T) -> Self {
        let (bus_sender, bus_receiver) = WATCHChannel(initial_value);
        MessageBus {
            sender: bus_sender,
            receiver: bus_receiver,
        }
    }
}

use tokio::sync::broadcast::{channel as BROADCASTChannel, Receiver, Sender};

#[derive(Debug)]
pub struct ClientBus<T: Clone> {
    pub sender: Sender<T>,
    pub receiver: Receiver<T>,
}

impl<T: Clone> ClientBus<T> {
    pub fn new() -> Self {
        let (bus_sender, bus_receiver) = BROADCASTChannel::<T>(1);
        ClientBus {
            sender: bus_sender,
            receiver: bus_receiver,
        }
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
    pub sender: String,
    pub destination: String,
    pub command: String,
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

    pub fn format_msg(mut self, msg: String) -> Self {
        if !msg.starts_with('@') {
            self.destination = "broadcast".to_string();
            self.payload = msg;
        } else if msg.contains(' ') {
            let (destination, payload) = msg.split_once(' ').unwrap();
            self.destination = destination.to_string()[1..].to_string();
            self.payload = payload.to_string();
        }
        self
    }
}

impl fmt::Display for BusMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        writeln!(f, "{:#?}", self)
    }
}
