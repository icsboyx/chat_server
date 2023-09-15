#[allow(unused_imports)]
use crate::gen_def::*;
use crate::*;

// let mut busses: HashMap<String, ClientBus<BusMessage>> = HashMap::new();

pub async fn start_chat_engine(busses: Sender<BusMessage>) {
    let mut local_receiver = busses.subscribe();
    loop {
        let message = local_receiver.recv().await.unwrap();
        println!("Bus received: {:#?}", message);
        if !message.command.is_empty() {
            check_command(message.command);
        }
    }
}
// loop {
//     for

//     let r = rx.recv().await.unwrap();
//     println!("Bus received: {:?}", r);
// }

fn check_command(command: String) {}
