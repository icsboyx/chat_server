use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::broadcast::{Receiver, Sender};
mod gen_def;
use gen_def::*;
mod chat_engine;

use chat_engine::*;

#[tokio::main]
async fn main() {
    let mut handles = Vec::new();
    let client_bus = ClientBus::new();

    handles.push(tokio::spawn(start_chat_engine(client_bus.receiver)));

    handles.push(tokio::spawn(async move {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:23456")
            .await
            .unwrap();
        loop {
            let (stream, _) = listener.accept().await.unwrap();
            {
                tokio::spawn(process(stream, client_bus.sender.clone()));
            }
        }
    }));

    for handle in handles {
        println!("GOT {:#?}", handle.await);
    }
}

async fn process(stream: TcpStream, client_bus: Sender<BusMessage>) {
    let id = stream.peer_addr().unwrap().to_string();
    let mut msg = BusMessage::new();

    let mut local_subscriber = client_bus.clone().subscribe();
    msg.sender = id.clone();

    let (mut stream_read, mut stream_write) = stream.into_split();

    let mut buf = vec![0; 1024];
    let mut join_msg = msg.clone();

    join_msg.command = "Client Joined)".to_string();
    client_bus.send(join_msg).unwrap();

    tokio::spawn(async move {
        loop {
            // let payload = rx.recv().await.unwrap();
            let payload = local_subscriber.recv().await.unwrap();
            stream_write
                .write_all(payload.to_string().as_bytes())
                .await
                .unwrap();
        }
    });

    loop {
        let n = stream_read
            .read(&mut buf)
            .await
            .expect("failed to read data from stream");
        let payload = msg
            .clone()
            .format_msg(String::from_utf8_lossy(&buf[..n]).to_string());
        client_bus.send(payload).unwrap();
        if n == 0 {
            return;
        }
    }
}
