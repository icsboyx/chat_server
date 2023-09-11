use bus::{Bus, BusReader};

use crate::*;
use std::{
    io::{self, Read, Write},
    net::TcpStream,
    time::Duration,
};

pub fn start_client(
    mut client: TcpStream,
    client_tx_bus: CommunicationBus<String>,
    client_rx_bus: BusReader<String>,
) {
    let mut bus_message = BusMessage::new();
    bus_message.sender = client.remote_id();
    new_thread!(client.remote_id(), {
        let mut buffer = [0; 1024];
        client_tx_bus
            .sender
            .send(format!("New client connected: {}", client.remote_id()))
            .unwrap();
        loop {
            client.set_nonblocking(true).unwrap();
            match client.read(&mut buffer) {
                Ok(bytes_read) => {
                    let payload = &buffer[0..bytes_read];
                    if bytes_read == 0 {
                        // Connection closed by the client
                        client.shutdown(std::net::Shutdown::Both).unwrap();
                        break;
                    }
                    if payload == [255, 244, 255, 253, 6] {
                        println!("Client {} disconnected.", client.remote_id());
                        client.shutdown(std::net::Shutdown::Both).unwrap();
                        break;
                    }
                    let string_payload = String::from_utf8(payload.to_vec()).unwrap();
                    println!("[{}]{}", client.remote_id(), string_payload);
                    let (mut d, mut p) = string_payload.split_once('/').unwrap_or(("", ""));
                    if d.is_empty() && p.is_empty() {
                        d = "broadcast";
                        p = string_payload.as_str();
                    };
                    bus_message.command = "chat".to_string();
                    bus_message.destination = d.to_string();
                    bus_message.payload = p.to_string();
                    client_tx_bus.sender.send(bus_message.msg()).unwrap();
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    // if let Ok(msg) = &bus.receiver.recv_timeout(Duration::from_secs(0)) {
                    //     let message = msg.clone();
                    //     println! {"Inside: {}", message}
                    //     if message.contains("BROADCAST")
                    //         && !message.contains(client.remote_id().as_str())
                    //     {
                    //         client.write_all(message.as_bytes()).unwrap();
                    //     }
                    // }
                    pause!(1);
                }
                Err(e) => {
                    eprintln!("Error reading from client: {}", e);
                    break;
                }
            }
        }
    });
}
