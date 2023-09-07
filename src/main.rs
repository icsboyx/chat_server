use bus::Bus;
use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
mod general_def;
use general_def::*;
mod clients;
mod msg_router;
use msg_router::*;

fn main() {
    let mut bus: Bus<DynamicValue> = Bus::new(10);

    msg_router(&mut bus);

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
            bus.broadcast(DynamicValue::Client(Client {
                stream_id: remote_id.clone(),
                stream: cloned_stream.arc_mutex(),
            }));
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
