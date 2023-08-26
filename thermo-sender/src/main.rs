use rand::Rng;
use std::net::UdpSocket;
use std::thread;
use std::time::Duration;

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind UDP socket");
    let remote_addr = "127.0.0.1:5555"; // Адрес и порт для отправки данных
    println!("Connected to {}", remote_addr);

    let mut rng = rand::thread_rng();
    let thermo_name = format!("Thermo-{}", rng.gen_range(1..=10));

    loop {
        // Генерируем случайное значение температуры для лета
        let temperature = rng.gen_range(25.0..35.0);
        let temperature_str = format!("{}: {:.2}", thermo_name, temperature);

        // Отправляем данные по UDP
        socket
            .send_to(temperature_str.as_bytes(), remote_addr)
            .expect("Failed to send data");
        println!("Sent data: {}", temperature_str);

        // Ждем некоторое время перед отправкой следующих данных
        thread::sleep(Duration::from_secs(2));
    }
}
