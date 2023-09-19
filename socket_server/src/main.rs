use std::sync::Arc;

use socket_client::{Command, Response};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    sync::Mutex,
};

use rand::Rng;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:7890")
        .await
        .expect("can't bind tcp listener");

    let smart_socket = Arc::new(Mutex::new(SmartSocket::default()));
    println!("Waiting for connections... (Ctrl-C to stop)");

    while let Ok((mut stream, addr)) = listener.accept().await {
        let host = addr.to_string();
        println!("Host '{host}' connected");

        let smart_socket = smart_socket.clone();
        tokio::spawn(async move {
            let mut in_buffer = [0u8];
            while stream.read_exact(&mut in_buffer).await.is_ok() {
                let response = smart_socket
                    .lock()
                    .await
                    .process_command(in_buffer[0].into());
                let response_buf: [u8; 5] = response.into();
                if stream.write_all(&response_buf).await.is_err() {
                    break;
                };
            }

            println!("Connection with {host} lost. Waiting for new connections...");
        });
    }
}

#[derive(Default)]
struct SmartSocket {
    enabled: bool,
}

impl SmartSocket {
    fn process_command(&mut self, cmd: Command) -> Response {
        match cmd {
            Command::TurnOn => {
                self.enabled = true;
                Response::Ok
            }
            Command::TurnOff => {
                self.enabled = false;
                Response::Ok
            }
            Command::GetStatus => {
                if self.enabled {
                    Response::Enabled
                } else {
                    Response::Disabled
                }
            }
            Command::GetPower => {
                if self.enabled {
                    let power: u32 = rand::thread_rng().gen_range(210.0..=230.0) as u32;
                    Response::Power(power)
                } else {
                    Response::Power(0)
                }
            }
            Command::Unknown => {
                println!("Unknown command received");
                Response::Unknown
            }
        }
    }
}
