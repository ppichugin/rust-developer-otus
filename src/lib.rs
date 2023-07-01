pub mod smart_home {
    pub struct SmartHouse {
        rooms: Vec<Room>,
    }

    impl Default for SmartHouse {
        fn default() -> Self {
            Self::new()
        }
    }

    impl SmartHouse {
        pub fn new() -> Self {
            let rooms = vec![
                Room {
                    name: String::from("Living Room"),
                    devices: vec![String::from("TV"), String::from("Lights")],
                },
                Room {
                    name: String::from("Kitchen"),
                    devices: vec![String::from("Oven"), String::from("Microwave")],
                },
            ];
            SmartHouse { rooms }
        }

        pub fn get_rooms(&self) -> Vec<String> {
            self.rooms.iter().map(|room| room.name.clone()).collect()
        }

        pub fn devices(&self, room: &str) -> Option<Vec<String>> {
            self.rooms
                .iter()
                .find(|r| r.name == room)
                .map(|r| r.devices.clone())
        }

        pub fn create_report(&self, info_provider: &(dyn DeviceInfoProvider + 'static)) -> String {
            let mut report = String::new();
            for room in &self.rooms {
                report += &format!("Room: {}\n", room.name);
                if let Some(devices) = self.devices(&room.name) {
                    for device in devices {
                        let device_info = info_provider.device_info(&room.name, &device);
                        match device_info {
                            Some(info) => report += &format!(" - {}: {}\n", device, info),
                            None => report += &format!(" - {}: Device not found\n", device),
                        }
                    }
                }
                report += "\n";
            }
            report
        }
    }

    struct Room {
        name: String,
        devices: Vec<String>,
    }

    pub trait DeviceInfoProvider {
        fn device_info(&self, room: &str, device: &str) -> Option<String>;
    }
}

pub mod device_providers {
    use super::smart_home::DeviceInfoProvider;

    pub struct SmartSocket {}

    impl DeviceInfoProvider for SmartSocket {
        fn device_info(&self, _room: &str, device: &str) -> Option<String> {
            match device {
                "TV" => Some("Power: On".to_string()),
                "Lights" => Some("Brightness: 80%".to_string()),
                _ => None,
            }
        }
    }

    pub struct SmartThermometer {}

    impl DeviceInfoProvider for SmartThermometer {
        fn device_info(&self, _room: &str, device: &str) -> Option<String> {
            match device {
                "Oven" => Some("Temperature: 180°C".to_string()),
                "Microwave" => Some("Power: 800W".to_string()),
                _ => None,
            }
        }
    }
}
