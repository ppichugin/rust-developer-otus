use crate::smart_socket::ImitationSmartSocket;
use crate::ADDR;
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

pub fn start_tcp_server() {
    let listener = TcpListener::bind(ADDR).expect("Failed to bind");
    println!("TCP server started\nListening on: {}\n", ADDR);

    let imitation_response = "Power: On\nConsumption: 100W";
    let imitation_socket = Arc::new(Mutex::new(ImitationSmartSocket::new(imitation_response)));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let imitation_socket = Arc::clone(&imitation_socket);
                thread::spawn(move || {
                    manage_response(stream, imitation_socket);
                });
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}

fn manage_response(mut stream: TcpStream, imitation_socket: Arc<Mutex<ImitationSmartSocket>>) {
    let mut buffer = [0; 1024];
    while let Ok(bytes_read) = stream.read(&mut buffer) {
        if bytes_read == 0 {
            // Клиент закрыл соединение
            println!("Client {} disconnected", stream.peer_addr().unwrap());
            break;
        }

        let command = String::from_utf8_lossy(&buffer[0..bytes_read])
            .trim()
            .to_lowercase();

        match command.as_str() {
            "turn_on" => {
                let mut imitation_socket = imitation_socket.lock().expect("Mutex lock failed");
                imitation_socket.turn_on();
                println!("Response: {}", imitation_socket.response);
            }
            "turn_off" => {
                let mut imitation_socket = imitation_socket.lock().expect("Mutex lock failed");
                imitation_socket.turn_off();
                println!("Response: {}", imitation_socket.response);
            }
            _ => {
                println!("Unknown command: {}", command);
            }
        }
    }
}
