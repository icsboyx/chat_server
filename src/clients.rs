use std::{io::Read, net::TcpStream, thread};

use crate::general_def::*;

// pub fn clients_listener(bus: &mut Bus<DynamicValue>) {
//     let mut bus_rx = bus.add_rx();
//     let msg_bus = thread::Builder::new().name("clients_listener".to_string());
//     msg_bus
//         .spawn(move || {
//             while let Ok(DynamicValue::Client(mut msg)) = bus_rx.recv() {
//                 println!("[CLIENTS_LISTENER]: {:#?}", msg);
//                 thread::Builder::new()
//                     .name(msg.stream_id)
//                     .spawn(move || start_client(&mut msg.stream))
//                     .unwrap();
//             }
//         })
//         .unwrap();
// }

pub fn start_client(client: &mut TcpStream, bus: &mut MessageBus<DynamicValue>) {
    let bus_clone = bus.clone();
    let mut client_clone = client.try_clone().unwrap();

    thread::Builder::new()
        .name(client.remote_id())
        .spawn(move || loop {
            let mut buffer = [0u8; 1024];
            match client_clone.read(&mut buffer) {
                Ok(bytes_read) => {
                    let payload = &buffer[0..bytes_read];
                    if bytes_read == 0 {
                        // Connection closed by the client
                        client_clone.shutdown(std::net::Shutdown::Both).unwrap();
                        break;
                    }
                    let string_payload = String::from_utf8(payload.to_vec()).unwrap();
                    bus_clone
                        .sender
                        .send(DynamicValue::ChatRawMSG(RawMessage {
                            sender: client_clone.remote_id(),
                            payload: string_payload,
                        }))
                        .unwrap();
                    if payload == [255, 244, 255, 253, 6] {
                        println!("Client sent: ^C");
                        client_clone.shutdown(std::net::Shutdown::Both).unwrap();
                        break;
                    }
                }
                Err(e) => {
                    eprintln!("Error reading from client: {}", e);
                    break;
                }
            }
        })
        .unwrap();
}
