#[allow(unused_imports)]
use crate::gen_def::*;
use crate::*;

// let mut busses: HashMap<String, ClientBus<BusMessage>> = HashMap::new();

pub async fn start_chat_engine(busses: HashMap<String, Sender<BusMessage>>) {
    loop {
        for (id, bus) in busses.into_iter() {
            let mut local_subscriber = bus.subscribe();
            tokio::spawn(async move {
                loop {
                    let message = local_subscriber.recv().await.unwrap();
                    println!("Bus {} received: {:?}", id, message);
                }
            });
        }
    }
}

// loop {
//     for

//     let r = rx.recv().await.unwrap();
//     println!("Bus received: {:?}", r);
// }
