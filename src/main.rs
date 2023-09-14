use tokio::io::{ AsyncReadExt, AsyncWriteExt };
use tokio::net::TcpStream;
use tokio::sync::broadcast::{ self, Sender, Receiver };
mod gen_def;
use gen_def::*;
mod chat_engine;
use chat_engine::*;

#[tokio::main]
async fn main() {
    let mut handles = Vec::new();
    let bus: (Sender<BusMessage>, Receiver<BusMessage>) = broadcast::channel(100);
    handles.push(tokio::spawn(start_chat_engine(bus.1)));
    handles.push(
        tokio::spawn(async move {
            let listener = tokio::net::TcpListener::bind("127.0.0.1:23456").await.unwrap();
            loop {
                let (stream, _) = listener.accept().await.unwrap();
                {
                    tokio::spawn(process(stream, bus.0.clone()));
                }
            }
        })
    );

    for handle in handles {
        println!("GOT {:#?}", handle.await);
    }
}

async fn process(stream: TcpStream, tx: Sender<BusMessage>) {
    let id = stream.peer_addr().unwrap().to_string();
    let mut msg = BusMessage::new();
    msg.sender = id.clone();

    let (mut stream_read, mut stream_write) = stream.into_split();

    let mut rx = tx.subscribe();
    let mut buf = vec![0; 1024];
    let mut join_msg = msg.clone();
    join_msg.command = "CLient Joined)".to_string();
    tx.send(join_msg).unwrap();

    tokio::spawn(async move {
        loop {
            let payload = rx.recv().await.unwrap();
            if payload.sender != id {
                stream_write.write_all(payload.to_string().as_bytes()).await.unwrap();
            }
        }
    });

    loop {
        let n = stream_read.read(&mut buf).await.expect("failed to read data from stream");
        let payload = msg.clone().format_msg(String::from_utf8_lossy(&buf[..n]).to_string());
        tx.send(payload).expect("send failed");
        if n == 0 {
            return;
        }
    }
}
