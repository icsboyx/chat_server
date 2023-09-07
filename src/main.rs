use bus::Bus;
use crossbeam_channel::{unbounded, Receiver, Sender};
use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
mod general_def;
use general_def::*;
mod clients;
use clients::*;

fn main() {
    let mut _bus: Bus<DynamicValue> = Bus::new(10);
    let (bus_tx, bus_rx): (Sender<DynamicValue>, Receiver<DynamicValue>) = unbounded();

    let listener = TcpListener::bind("127.0.0.1:23456").expect("Failed to bind");
    let mut active_clients: HashMap<String, TcpStream> = HashMap::new();
    let bus_tx_clone = bus_tx.clone();
    handle_clients(bus_rx.clone());

    for stream_result in listener.incoming() {
        if let Ok(stream) = stream_result {
            let cloned_stream = stream.try_clone().expect("Failed to clone stream");
            let remote_id = stream.remote_id();
            active_clients.insert(stream.remote_id(), stream);
            // thread::spawn(move || {
            //     handle_client(&mut stream);
            // });
            // active_clients.insert(remote_id, cloned_stream);
            bus_tx_clone
                .send(DynamicValue::Client(Client {
                    stream_id: remote_id,
                    stream: cloned_stream,
                }))
                .unwrap();
        } else {
            eprintln!("Failed to accept a client connection");
        }
    }
}
