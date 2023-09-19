use std::{net::SocketAddr, time::Duration};

use tokio::{
    net::UdpSocket,
    time::{self, Instant},
};

use rand::Rng;

#[tokio::main]
async fn main() {
    let receiver_address = "127.0.0.1:4321";
    println!("Receiver address: {}", receiver_address);

    let client = receiver_address
        .parse::<SocketAddr>()
        .expect("valid socket address expected");

    let srv_bind_addr = "127.0.0.1:4320";
    let socket = UdpSocket::bind(srv_bind_addr)
        .await
        .expect("can't bind socket");
    let temperature_generator = TemperatureGenerator::default();

    println!("Start sending temperature from {srv_bind_addr} to {client}");

    loop {
        let temperature = temperature_generator.generate();
        let bytes = temperature.to_be_bytes();
        let send_result = socket.send_to(&bytes, client).await;
        if let Err(err) = send_result {
            println!("can't send temperature: {err}")
        }
        println!("Sent temperature, {:?} to {}", temperature, client);

        time::sleep(Duration::from_secs(1)).await;
    }
}

struct TemperatureGenerator {
    started: Instant,
}

impl Default for TemperatureGenerator {
    fn default() -> Self {
        Self {
            started: Instant::now(),
        }
    }
}

impl TemperatureGenerator {
    pub fn generate(&self) -> f32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(18.0..=22.2)
    }
}

impl Drop for TemperatureGenerator {
    fn drop(&mut self) {
        let new_now = Instant::now();
        println!(
            "Temperature generator worked during {:?}",
            new_now.checked_duration_since(self.started)
        );
    }
}
