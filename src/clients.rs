use crate::general_def::*;
use crossbeam_channel::Receiver;
use std::{io::Read, net::TcpStream, thread};
//
pub fn handle_clients(bus_rx: Receiver<DynamicValue>) {
    // Read from the client and print incoming messages
    thread::spawn(move || loop {
        while let Ok(DynamicValue::Client(mut client)) = bus_rx.recv() {
            println!("New Connection from: {:#?}", client);
            thread::spawn(move || start_client(&mut client.stream));
        }
    });
}
pub fn start_client(client: &mut TcpStream) {
    let mut buffer = [0; 1024];
    loop {
        match client.read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    // Connection closed by the client
                    break;
                }
                let message = String::from_utf8_lossy(&buffer[0..bytes_read]);
                println!(
                    "Received message from {}: {}",
                    client.peer_addr().unwrap(),
                    message
                );
            }
            Err(e) => {
                eprintln!("Error reading from client: {}", e);
                break;
            }
        }
    }
}
