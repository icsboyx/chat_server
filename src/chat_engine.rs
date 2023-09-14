use tokio::sync::broadcast::Receiver;
use crate::*;

pub async fn start_chat_engine(mut rx: Receiver<BusMessage>) {
    loop {
        let r = rx.recv().await.unwrap();
        println!("Bus received: {:?}", r);
    }
}
