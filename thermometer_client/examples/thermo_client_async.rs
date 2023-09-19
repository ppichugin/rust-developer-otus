use std::time::Duration;
use thermometer_client::SmartThermo;
use tokio::time;

#[tokio::main]
async fn main() {
    let receiver_address = "127.0.0.1:4321";
    let thermometer = SmartThermo::new(receiver_address).await.unwrap();
    for _ in 0..120 {
        let temperature = thermometer.get_temperature().await;
        time::sleep(Duration::from_secs(1)).await;
        println!("The temperature is {temperature}");
    }
}
