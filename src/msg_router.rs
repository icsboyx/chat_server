use crate::{clients::start_client, general_def::*};
use bus::Bus;
use std::thread;

pub fn msg_router(bus: &mut Bus<DynamicValue>) {
    let mut bus_rx = bus.add_rx();
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
                    DynamicValue::ChatMsg(_payload) => {
                        todo!();
                    }
                }
            }
        })
        .unwrap();
}
