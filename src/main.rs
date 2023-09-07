use crossbeam_channel::{unbounded, Receiver, Sender};
use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
mod general_def;
use general_def::*;
mod clients;
mod msg_router;
use msg_router::*;

fn main() {
    let (bus_sender, bus_reader): (Sender<DynamicValue>, Receiver<DynamicValue>) = unbounded();
    let bus = MessageBus {
        sender: bus_sender,
        receiver: bus_reader,
    };

    // clients_listener(&mut bus);
    msg_router(&mut bus.clone());

    let listener = TcpListener::bind("127.0.0.1:23456").expect("Failed to bind");
    let mut active_clients: HashMap<String, TcpStream> = HashMap::new();

    // Start Listener for incoming connections
    for stream_result in listener.incoming() {
        if let Ok(stream) = stream_result {
            let cloned_stream = stream.try_clone().expect("Failed to clone stream");
            let remote_id = stream.remote_id();
            // Add client to Client HashMap
            active_clients.insert(stream.remote_id(), stream);

            //Sen Client connection on crossbeam_channel
            bus.sender
                .send(DynamicValue::Client(Client {
                    stream_id: remote_id.clone(),
                    stream: cloned_stream.arc_mutex(),
                }))
                .unwrap();
            // bus_tx_clone
            //     .send(DynamicValue::Client(Client {
            //         stream_id: remote_id.clone(),
            //         stream: cloned_stream.arc_mutex(),
            //     }))
            //     .unwrap();
        } else {
            eprintln!("Failed to accept a client connection");
        }
    }
}
