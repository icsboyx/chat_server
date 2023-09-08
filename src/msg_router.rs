use crate::{clients::start_client, general_def::*};
use std::thread;

pub fn msg_router(bus: &mut MessageBus<DynamicValue>) {
    let bus_clone = bus.clone();
    let mut client_bus_clone = bus.clone();
    thread::Builder::new()
        .name("msg_router".to_string())
        .spawn(move || {
            while let Ok(msg) = bus_clone.receiver.recv() {
                println!("[MSG_ROUTER]:{:#?}", msg);
                match msg {
                    DynamicValue::Client(payload) => {
                        start_client(&mut payload.stream.lock().unwrap(), &mut client_bus_clone);
                    }
                    DynamicValue::ChatMsg(_payload) => {}
                    DynamicValue::ChatRawMSG(_payload) => {}
                }
            }
        })
        .unwrap();
}

// thread::Builder::new().name("msg_bus".to_string()).spawn(move|| {
//     while let Ok(msg) =  {
//         println!("[MSG_ROUTER]: {:#?}", msg);
//         match msg {
//             DynamicValue::Client(mut msg) => {}
//             DynamicValue::ChatMsg(_payload) => {}
//         }
