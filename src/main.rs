use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::select;
use tokio::sync::broadcast::Sender;

mod gen_def;
use gen_def::*;
mod chat_engine;

use chat_engine::*;

#[tokio::main]
async fn main() {
    let mut handles = Vec::new();
    let client_bus = ClientBus::new();

    handles.push(tokio::spawn(start_chat_engine(client_bus.sender.clone())));

    handles.push(tokio::spawn(async move {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:23456")
            .await
            .unwrap();
        loop {
            let (stream, _) = listener.accept().await.unwrap();
            tokio::spawn(process(stream, client_bus.sender.clone()));
        }
    }));

    for handle in handles {
        println!("END GOT {:#?}", handle.await);
    }
}

async fn process(stream: TcpStream, bus_tx: Sender<BusMessage>) {
    let id = stream.peer_addr().unwrap().to_string();
    let mut msg = BusMessage::new();

    let mut local_receiver = bus_tx.subscribe();
    let local_sender = bus_tx;
    msg.sender = id.clone();

    let (mut stream_read, mut stream_write) = stream.into_split();

    let mut buf = vec![0; 1024];
    let mut join_msg = msg.clone();
    join_msg.command = "Client Joined)".to_string();
    local_sender.send(join_msg).unwrap();

    loop {
        select! {
        payload = local_receiver.recv() => {
            let payload = payload.unwrap();
            if (payload.destination == id.clone() || payload.destination == "broadcast")
                && payload.sender != id.clone()
            {
                stream_write
                    .write_all(payload.to_string().as_bytes())
                    .await
                    .unwrap();
            }
        }
        n = stream_read.read(&mut buf) => {{
            let bytes_read = n.unwrap();
            println!("#### {:#?}", &buf[..bytes_read]);
        if buf[..bytes_read] == [255,244,255,253,6] {
                println!("Client {} disconnected.", stream_read.peer_addr().unwrap());
                break;
            }
            let payload = msg.clone().format_msg(
                String::from_utf8_lossy(&buf[..bytes_read])
                    .to_string()
                    .replace("\r\n", ""),
            );
            local_sender.send(payload).unwrap();
            if bytes_read == 0 {
            }}
        }}
    }
}
