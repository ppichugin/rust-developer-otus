mod smart_socket;

use std::io::{Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

const ADDR: &str = "192.168.0.103:8095";

fn main() {
    let server_thread = thread::spawn(|| {
        start_tcp_server();
    });

    // Create a SmartSocketClient instance
    let mut socket_client = smart_socket::SmartSocketClient::new(ADDR);

    let simulation_thread = thread::spawn(move || {
        simulate_smart_socket_behavior(&mut socket_client);
    });

    server_thread.join().unwrap();
    simulation_thread.join().unwrap();
}

fn start_tcp_server() {
    let listener = TcpListener::bind(ADDR).expect("Failed to bind");
    println!("TCP server started");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let imitation_response = "Power: On\nConsumption: 100W";
    let imitation_socket = smart_socket::ImitationSmartSocket::new(imitation_response);

    for _ in 0..5 {
        let response = imitation_socket.get_info();
        stream.write_all(response.as_bytes()).expect("Failed to send response");
        thread::sleep(Duration::from_secs(2));
    }
}

fn simulate_smart_socket_behavior(socket_client: &mut smart_socket::SmartSocketClient) {
    if let Err(err) = socket_client.connect() {
        eprintln!("Error connecting to the smart socket: {}", err);
        return;
    }

    for _ in 0..5 {
        if let Err(err) = socket_client.send_command("turn_on") {
            eprintln!("Error sending command: {}", err);
            return;
        }

        println!("Smart socket is on");

        thread::sleep(Duration::from_secs(2));

        if let Err(err) = socket_client.send_command("turn_off") {
            eprintln!("Error sending command: {}", err);
            return;
        }

        println!("Smart socket is off");

        thread::sleep(Duration::from_secs(2));
    }
}