use std::io::{Read, Write};
use std::net::TcpStream;

pub struct SmartSocketClient {
    address: String,
    stream: Option<TcpStream>,
}

impl SmartSocketClient {
    pub fn new(address: &str) -> Self {
        SmartSocketClient {
            address: address.to_string(),
            stream: None,
        }
    }

    pub fn connect(&mut self) -> Result<(), std::io::Error> {
        let stream = TcpStream::connect(&self.address)?;
        self.stream = Some(stream);
        Ok(())
    }

    pub fn send_command(&mut self, command: &str) -> Result<String, std::io::Error> {
        if let Some(stream) = &mut self.stream {
            stream.write_all(command.as_bytes())?;
            stream.flush()?; // Flush the stream to ensure data is sent

            let mut response = String::new();
            stream.read_to_string(&mut response)?;

            Ok(response)
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotConnected,
                "Socket not connected",
            ))
        }
    }
}

pub struct ImitationSmartSocket {
    response: String,
}

impl ImitationSmartSocket {
    pub fn new(response: &str) -> Self {
        ImitationSmartSocket {
            response: response.to_string(),
        }
    }

    pub fn get_info(&self) -> &str {
        &self.response
    }
}
