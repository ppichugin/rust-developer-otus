mod smart_socket;
mod tcp_server;

use rand::Rng;
use std::io::Write;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

const ADDR: &str = "127.0.0.1:8095";

fn main() {
    let server_thread = thread::spawn(|| {
        tcp_server::start_tcp_server();
    });

    let client_thread = thread::spawn(|| {
        simulate_smart_socket_behavior();
    });

    server_thread.join().unwrap();
    client_thread.join().unwrap();
}

fn simulate_smart_socket_behavior() {
    let mut socket_client = TcpStream::connect(ADDR).expect("Failed to connect to server");
    println!(
        "Socket client started: {}",
        socket_client.peer_addr().unwrap()
    );

    for ind in 0..5 {
        let command = if rand::thread_rng().gen_bool(0.5) {
            "turn_on"
        } else {
            "turn_off"
        };
        println!("{}: Sending command '{}'... ", ind, command);
        socket_client
            .write_all(command.as_bytes())
            .expect("Failed to send command");
        socket_client.flush().expect("Failed to flush");
        thread::sleep(Duration::from_secs(2));
    }

    // test unknown command
    socket_client
        .write_all("ABCD".as_bytes())
        .expect("Failed to send command");
    socket_client.flush().expect("Failed to flush");
    thread::sleep(Duration::from_secs(2));

    // close the connection
    println!(
        "Socket client {} finished",
        socket_client.peer_addr().unwrap()
    );
    drop(socket_client);
}
