use std::net::UdpSocket;
use std::sync::mpsc::{self};
use std::thread;
use std::time::Duration;

const TIME_OUT: u64 = 10; // Тайм-аут в секундах
fn main() {
    // Создаем UDP сокет для приема данных
    let socket = UdpSocket::bind("127.0.0.1:5555").expect("Failed to bind UDP socket");
    println!("Listening on {}", socket.local_addr().unwrap());

    // Канал для уведомления о наличии активных соединений
    let (tx, rx) = mpsc::channel();

    // Запускаем поток для приема данных
    thread::spawn(move || {
        let mut buffer = [0; 16]; // Размер буфера для приема данных

        loop {
            let (num_bytes, _) = socket
                .recv_from(&mut buffer)
                .expect("Failed to receive data");

            let received_data = String::from_utf8_lossy(&buffer[0..num_bytes]);
            println!("Received data: {}", received_data);

            // Уведомляем основной поток о наличии активности
            tx.send(true).expect("Failed to send activity");
        }
    });

    // Пытаемся считать данные из канала в течение тайм-аута
    while let Ok(active) = rx.recv_timeout(Duration::from_secs(TIME_OUT)) {
        if !active {
            break;
        }
    }

    println!("No activity for {} seconds. Closing socket...", TIME_OUT);
}
