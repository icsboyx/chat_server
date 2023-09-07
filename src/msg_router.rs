use crate::{clients::start_client, general_def::*};
use bus::BusReader;
use std::thread;

pub fn msg_router(mut bus_rx: BusReader<DynamicValue>) {
    let msg_bus = thread::Builder::new().name("msg_bus".to_string());
    msg_bus
        .spawn(move || {
            while let Ok(msg) = bus_rx.recv() {
                println!("[MSG_ROUTER]: {:#?}", msg);
                match msg {
                    DynamicValue::Client(mut payload) => {
                        thread::Builder::new()
                            .name(payload.stream_id)
                            .spawn(move || start_client(&mut payload.stream))
                            .unwrap();
                    }
                    DynamicValue::ChatMsg(payload) => {
                        todo!();
                    }
                }
            }
        })
        .unwrap();
}
