use mini_redis::Frame;
use tokio::io::{ AsyncReadExt, AsyncWriteExt };
use tokio::net::TcpStream;
use tokio::sync::broadcast::{ self, Sender, Receiver };
use tokio::sync::mpsc::{ self, UnboundedReceiver, UnboundedSender };
use tokio::time::Duration;
mod chat_engine;
use chat_engine::*;

#[tokio::main]
async fn main() {
    let mut handles = Vec::new();
    let bus: (Sender<String>, Receiver<String>) = broadcast::channel(16);
    let client_rx = bus.0.subscribe();

    handles.push(tokio::spawn(start_chat_engine(bus.1)));
    handles.push(
        tokio::spawn(async move {
            let listener = tokio::net::TcpListener::bind("127.0.0.1:23456").await.unwrap();
            while let Ok((socket, _)) = listener.accept().await {
                tokio::spawn(process(socket, bus.0.clone()));
            }
        })
    );

    for handle in handles {
        println!("GOT {:#?}", handle.await);
    }
}

async fn process(mut socket: TcpStream, tx: Sender<String>) {
    let mut rx = tx.subscribe();
    let mut buf = vec![0; 1024];
    let id = socket.peer_addr().unwrap();
    // tokio::spawn();
    tokio::spawn(async move {
        loop {
            let value = rx.recv().await.unwrap();
            println!("Client {:#?}, received {}", id.to_string(), value);
        }
    });
    loop {
        let n = socket.read(&mut buf).await.expect("failed to read data from socket");
        tx.send(String::from_utf8_lossy(&buf[..n]).to_string()).expect("send failed");
        if n == 0 {
            return;
        }
    }
}
