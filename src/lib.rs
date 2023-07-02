pub mod devices;

pub mod smart_home {
    use crate::devices::Devices::*;
    use crate::devices::{DeviceInfoProvider, Devices};

    pub struct SmartHouse {
        pub(crate) name: String,
        rooms: Vec<Room>,
    }

    impl SmartHouse {
        pub fn new(name: &str) -> Self {
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
            SmartHouse {
                name: name.to_string(),
                rooms,
            }
        }

        pub fn get_rooms(&self) -> [&str; 2] {
            self.rooms
                .iter()
                .map(|r| r.name.as_str())
                .collect::<Vec<&str>>()
                .try_into()
                .unwrap()
        }

        pub fn devices(&self, room: &str) -> Option<Vec<Devices>> {
            self.rooms
                .iter()
                .find(|r| r.name == room)
                .map(|r| r.devices.clone())
        }

        pub fn create_report<T>(&self, info_provider: &T) -> String
        where
            T: DeviceInfoProvider,
        {
            let mut report = String::new();
            report += &format!("Report for house: {}\n", self.name);
            report += &format!("Rooms total: {:?}\n", self.get_rooms().len());

            self.get_rooms().iter().for_each(|r| {
                report += &format!("Room: {}\n", r);
                if let Some(devices) = self.devices(r) {
                    for device in devices {
                        let device_info = info_provider.device_info(r, &device);
                        match device_info {
                            Some(info) => report += &format!(" - {:?}: {:?}\n", device, info),
                            None => report += &format!(" - {:?}: Device not found\n", device),
                        }
                    }
                }
                report += "\n";
            });
            report
        }
    }

    struct Room {
        name: String,
        devices: Vec<Devices>,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::devices::Devices;
    use smart_home::SmartHouse;

    #[test]
    fn get_new_house() {
        let house = SmartHouse::new("My House");
        assert_eq!(house.name, "My House");
    }

    #[test]
    fn house_get_rooms() {
        let house = SmartHouse::new("My House");
        assert_eq!(house.get_rooms(), ["Living Room", "Kitchen"]);
    }

    #[test]
    fn house_get_devices() {
        let house = SmartHouse::new("My House");
        assert_eq!(
            house.devices("Living Room"),
            Some(vec![Devices::TV, Devices::Lights, Devices::Thermometer])
        );
        assert_eq!(
            house.devices("Kitchen"),
            Some(vec![Devices::Oven, Devices::Microwave])
        );
        assert_eq!(house.devices("Bedroom"), None);
    }

    #[test]
    fn house_create_report() {
        let house = SmartHouse::new("My House");
        let socket = devices::SmartSocket {};
        let thermo = devices::SmartThermometer {};
        let report = house.create_report(&socket);
        assert_eq!(
            report,
            "Report for house: My House\nRooms total: 2\nRoom: Living Room\n - TV: \"Power: On\"\n - Lights: \"Brightness: 80%\"\n - Thermometer: Device not found\n\nRoom: Kitchen\n - Oven: \"Power: 2000W\"\n - Microwave: \"Power: 800W\"\n\n"
        );
        let report = house.create_report(&thermo);
        assert_eq!(
            report,
            "Report for house: My House\nRooms total: 2\nRoom: Living Room\n - TV: Device not found\n - Lights: Device not found\n - Thermometer: \"Temperature: 20C\"\n\nRoom: Kitchen\n - Oven: Device not found\n - Microwave: Device not found\n\n"
        );
    }
}
