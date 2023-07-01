use crate::device_providers::{SmartSocket, SmartThermometer};
use crate::smart_home::DeviceInfoProvider;
use Devices::*;

#[derive(Debug, Clone)]
pub enum Devices {
    TV,
    Lights,
    Oven,
    Microwave,
    Thermometer,
}

pub mod smart_home {
    use crate::Devices;
    use crate::Devices::*;

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
                    devices: vec![TV, Lights, Thermometer],
                },
                Room {
                    name: String::from("Kitchen"),
                    devices: vec![Oven, Microwave],
                },
            ];
            SmartHouse { rooms }
        }

        pub fn create_report<T>(&self, info_provider: &T) -> String
        where
            T: DeviceInfoProvider,
        {
            let mut report = String::new();
            for room in &self.rooms {
                report += &format!("Room: {}\n", room.name);
                if let Some(devices) = self.devices(&room.name) {
                    for device in devices {
                        let device_info = info_provider.device_info(&room.name, &device);
                        match device_info {
                            Some(info) => report += &format!(" - {:?}: {:?}\n", device, info),
                            None => report += &format!(" - {:?}: Device not found\n", device),
                        }
                    }
                }
                report += "\n";
            }
            report
        }

        fn devices(&self, room: &str) -> Option<Vec<Devices>> {
            self.rooms
                .iter()
                .find(|r| r.name == room)
                .map(|r| r.devices.clone())
        }
    }

    struct Room {
        name: String,
        devices: Vec<Devices>,
    }

    pub trait DeviceInfoProvider {
        fn device_info(&self, room: &str, device: &Devices) -> Option<String>;
    }
}

pub mod device_providers {
    use super::smart_home::DeviceInfoProvider;
    use crate::Devices;
    use crate::Devices::*;

    pub struct SmartSocket {}

    impl DeviceInfoProvider for SmartSocket {
        fn device_info(&self, _room: &str, device: &Devices) -> Option<String> {
            match device {
                TV => Some("Power: On".to_string()),
                Lights => Some("Brightness: 80%".to_string()),
                Oven => Some("Power: 2000W".to_string()),
                Microwave => Some("Power: 800W".to_string()),
                _ => None,
            }
        }
    }

    pub struct SmartThermometer {}

    impl DeviceInfoProvider for SmartThermometer {
        fn device_info(&self, _room: &str, device: &Devices) -> Option<String> {
            match device {
                Thermometer => Some("Temperature: 20C".to_string()),
                _ => None,
            }
        }
    }
}

pub struct OwningDeviceInfoProvider {
    pub socket: SmartSocket,
}

impl DeviceInfoProvider for OwningDeviceInfoProvider {
    fn device_info(&self, room: &str, device: &Devices) -> Option<String> {
        match device {
            TV | Lights | Oven | Microwave => self.socket.device_info(room, device),
            _ => None,
        }
    }
}

pub struct BorrowingDeviceInfoProvider<'a, 'b> {
    pub socket: &'a SmartSocket,
    pub thermo: &'b SmartThermometer,
}

impl<'a, 'b> DeviceInfoProvider for BorrowingDeviceInfoProvider<'a, 'b> {
    fn device_info(&self, room: &str, device: &Devices) -> Option<String> {
        match device {
            TV | Lights | Oven | Microwave => self.socket.device_info(room, device),
            Thermometer => self.thermo.device_info(room, device),
        }
    }
}
