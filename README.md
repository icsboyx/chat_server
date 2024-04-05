# Chat Application

## Overview

This is a simple chat application written in Rust using the Tokio asynchronous runtime. It allows multiple clients to connect and exchange messages in real-time.

## Features

- Real-time chat with multiple clients.
- Supports basic chat commands.
- Easily extensible for further features.

## Getting Started

### Prerequisites

- Rust (https://www.rust-lang.org/)

### Installation

1. Clone this repository:

```bash
git clone https://github.com/yourusername/chat-application.git
cd chat-application
```
## Usage
Start the chat server:
```bash
cargo build; cargo run
```
Clients can connect to the server using a TCP client (e.g., telnet or a custom chat client).

Start sending and receiving messages in the chat.

## Chat Commands
* /list: Lists available commands (Add more commands as needed).
* /help: Displays information about available commands.

** Working in progress **


## Tags

- Chat
- Rust
- Tokio
- Asynchronous
- Messaging
- Networking
- Real-time
- CLI
- Chat Application


### Contribution
Contributions are welcome! If you find a bug or want to add new features, feel free to create a pull request.

## Special Thanks

This project was inspired by the Twitch channel of [Prof. Andrea Pollini](https://www.twitch.tv/profandreapollini) and the supportive Twitch community. Thanks to their encouragement and feedback!

## Acknowledgments
Thanks to the Rust community and the Tokio project for providing powerful asynchronous tools.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) for details.