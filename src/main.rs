use mini_redis::Frame;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
use tokio::time::Duration;

#[tokio::main]
async fn main() {
    let mut handles = Vec::new();
    let (tx, mut rx): (UnboundedSender<String>, UnboundedReceiver<String>) =
        mpsc::unbounded_channel();

    handles.push(tokio::spawn(async move {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:23456")
            .await
            .unwrap();
        while let Ok((socket, _)) = listener.accept().await {
            let tx_clone = tx.clone();
            tokio::spawn(async move {
                process(socket, tx_clone).await;
            });
        }
    }));

    handles.push(tokio::spawn(async move {
        while let Some(r) = rx.recv().await {
            println!("Bus received: {:?}", r);
        }
    }));

    for handle in handles {
        println!("GOT {:?}", handle.await.unwrap());
    }
}

async fn process(mut socket: TcpStream, tx: UnboundedSender<String>) {
    let mut buf = vec![0; 1024];
    loop {
        let n = socket
            .read(&mut buf)
            .await
            .expect("failed to read data from socket");
        tx.send(String::from_utf8_lossy(&buf[..n]).to_string())
            .expect("send failed");

        if n == 0 {
            return;
        }

        socket
            .write_all(&buf[..n])
            .await
            .expect("failed to write data to socket");
    }
}
