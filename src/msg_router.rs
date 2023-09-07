use crate::general_def::*;
use std::{sync::mpsc::Receiver, thread};

pub fn msg_router(receiver: Receiver<DynamicValueReturn>) {
    let msg_bus = thread::Builder::new().name("msg_bus".to_string());
    msg_bus
        .spawn(move || {
            for msg in receiver {
                match msg {
                    DynamicValueReturn::ClientREF(payload) => println!("{:#?}", payload),
                    DynamicValueReturn::ClientMSG(payload) => println!("{:#?}", payload),
                }
            }
        })
        .unwrap();
}
