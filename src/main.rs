use std::net::SocketAddr;

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast,
};

#[tokio::main]
async fn main() {
    // Async Chat
    let listener = TcpListener::bind("localhost:8080").await.unwrap();

    let (sender, receiver) = broadcast::channel(10);
    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();
        let sender = sender.clone();
        let mut receiver = sender.subscribe();

        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();
            let mut reader = BufReader::new(reader);
            let mut line = String::new();

            loop {
                tokio::select! {
                    _ = reader.read_line(&mut line) => {
                        sender.send((line.clone(), addr)).unwrap();
                        line.clear();
                    }
                    result = receiver.recv() => {
                        let (message, message_address) = result.unwrap();
                        if addr != message_address {
                            writer.write_all(message.as_bytes()).await.unwrap();
                        }
                    }

                }
            }
        });
    }
}
