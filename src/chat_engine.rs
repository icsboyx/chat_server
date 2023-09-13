use tokio::task::{ JoinError, JoinHandle };
use tokio::sync::broadcast::{ self, Sender, Receiver };

use crate::*;

pub async fn start_chat_engine(mut rx: Receiver<String>) {
    loop {
        while let r = rx.recv().await.unwrap() {
            println!("Bus received: {:?}", r);
        }
    }
}
