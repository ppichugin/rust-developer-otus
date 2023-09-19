use std::{
    error::Error,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use tokio::{
    net::{ToSocketAddrs, UdpSocket},
    sync::Mutex,
};

pub struct SmartThermo {
    temperature: Arc<Temperature>,
    finished: Arc<AtomicBool>,
}

impl SmartThermo {
    pub async fn new(address: impl ToSocketAddrs) -> Result<Self, Box<dyn Error>> {
        let socket = UdpSocket::bind(address).await?;
        let finished = Arc::new(AtomicBool::new(false));
        let temperature = Arc::new(Temperature::default());

        let temperature_clone = temperature.clone();
        let finished_clone = finished.clone();

        tokio::spawn(async move {
            loop {
                if finished_clone.load(Ordering::SeqCst) {
                    return;
                }

                let mut buf = [0; 4];
                socket.readable().await.unwrap();

                let mut received_len = 0;

                let buf_len = buf.len();
                while received_len < buf_len {
                    match socket.recv_from(&mut buf[received_len..buf_len]).await {
                        Ok((len, _)) => received_len += len,
                        Err(error) => {
                            println!("Can't receive datagram: {:?}", error);
                            break;
                        }
                    }
                }

                if buf.len() != 4 {
                    println!("incorrect datagram received");
                    continue;
                }

                let val = f32::from_be_bytes(buf);
                temperature_clone.set(val).await;
            }
        });

        Ok(Self {
            temperature,
            finished,
        })
    }

    pub async fn get_temperature(&self) -> f32 {
        self.temperature.get().await
    }
}

impl Drop for SmartThermo {
    fn drop(&mut self) {
        self.finished.store(true, Ordering::SeqCst)
    }
}

#[derive(Default)]
struct Temperature(Mutex<f32>);

impl Temperature {
    pub async fn get(&self) -> f32 {
        *self.0.lock().await
    }

    pub async fn set(&self, val: f32) {
        *self.0.lock().await = val
    }
}
