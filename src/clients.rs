use std::{
    io::Read,
    net::TcpStream,
    sync::{Arc, Mutex},
};
//

pub fn start_client(client: &mut Arc<Mutex<TcpStream>>) {
    let mut buffer = [0; 1024];
    loop {
        let mut client = client.lock().unwrap();
        match client.read(&mut buffer) {
            Ok(bytes_read) => {
                let payload = &buffer[0..bytes_read];
                if bytes_read == 0 {
                    // Connection closed by the client
                    client.shutdown(std::net::Shutdown::Both).unwrap();
                    break;
                }
                if payload == [255, 244, 255, 253, 6] {
                    println!("Client sent: ^C");
                    client.shutdown(std::net::Shutdown::Both).unwrap();
                    break;
                }
            }
            Err(e) => {
                eprintln!("Error reading from client: {}", e);
                break;
            }
        }
    }
}
