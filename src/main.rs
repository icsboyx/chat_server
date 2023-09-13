use tokio::io::{ AsyncReadExt, AsyncWriteExt };
use tokio::net::TcpStream;
use tokio::sync::broadcast::{ self, Sender, Receiver };

mod chat_engine;
use chat_engine::*;

#[tokio::main]
async fn main() {
    let mut handles = Vec::new();
    let bus: (Sender<String>, Receiver<String>) = broadcast::channel(100);
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

async fn process(stream: TcpStream, tx: Sender<String>) {
    let id = stream.peer_addr().unwrap().to_string();
    let (mut stream_read, mut stream_write) = stream.into_split();
    let mut rx = tx.subscribe();
    let mut buf = vec![0; 1024];

    tx.send(format!("Client {}, is now connected", id)).unwrap();

    tokio::spawn(async move {
        loop {
            let payload = rx.recv().await.unwrap();
            println!("Client {:#?}, received {}", id.to_string(), payload);
            stream_write.write_all(payload.as_bytes()).await.unwrap();
        }
    });
    loop {
        let n = stream_read.read(&mut buf).await.expect("failed to read data from stream");
        tx.send(String::from_utf8_lossy(&buf[..n]).to_string()).expect("send failed");
        if n == 0 {
            return;
        }
    }
}
