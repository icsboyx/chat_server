use std::{
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
        let (bus_sender, bus_receiver) = BROADCASTChannel::<T>(100);
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
        match msg {
            c if c.starts_with('@') && c.contains(' ') => {
                let (destination, payload) = c.split_once(' ').unwrap();
                self.destination = destination.to_string()[1..].to_string();
                self.payload = payload.to_string();
            }
            c if c.starts_with('/') => {
                self.command = c[1..].to_string();
            }
            _ => {
                self.destination = "broadcast".to_string();
                self.payload = msg;
            }
        }
        self
    }
}

impl fmt::Display for BusMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        writeln!(f, "{:#?}", self)
    }
}

pub enum Commands {
    Help,
    List(ListType),
    Error,
}
pub enum ListType {
    User,
    Channels,
}
