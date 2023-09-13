use tokio::sync::broadcast::Receiver;

pub async fn start_chat_engine(mut rx: Receiver<String>) {
    loop {
        let r = rx.recv().await.unwrap();
        println!("Bus received: {:?}", r);
    }
}
