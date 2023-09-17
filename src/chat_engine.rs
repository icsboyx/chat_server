use crate::*;

// let mut busses: HashMap<String, ClientBus<BusMessage>> = HashMap::new();

pub async fn start_chat_engine(bus_tx: Sender<BusMessage>) {
    let mut local_receiver = bus_tx.subscribe();
    let local_sender = bus_tx;
    loop {
        let message = local_receiver.recv().await.unwrap();
        println!("Bus received: {:#?}", message);
        if !message.command.is_empty() {
            let mut reply = BusMessage::new();
            reply.command = "".to_owned();
            reply.destination = message.sender;
            reply.sender = "SERVER".to_owned();
            reply.payload = check_command(message.command);
            local_sender.send(reply).unwrap();
        }
    }
}
// loop {
//     for

//     let r = rx.recv().await.unwrap();
//     println!("Bus received: {:?}", r);
// }

fn check_command(command: String) -> String {
    let command_list = vec!["list", "help"];
    let payload = command.to_lowercase().as_str().to_owned();
    match payload {
        _ if command_list[0] == payload => "list".to_string(),
        _ if command_list[1] == payload => command_list.join(" /"),
        _ => format!(
            "Wrong command, /{} not implemented. Use /help to display all commands",
            command
        ),
    }
}
