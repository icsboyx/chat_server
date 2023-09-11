use crate::*;

pub fn start_chat_engine(mut client_tx_bus: CommunicationBus<String>, client_rx_bus: &Bus<String>) {
    new_thread!("chat_engine", {
        loop {
            match client_tx_bus.receiver.recv() {
                Ok(message) => {
                    println!("[CHAT_ENGINE]:{:#?}", message);
                    client_rx_bus.broadcast(message);
                }

                Err(_) => todo!(),
            }
        }
    });
}
