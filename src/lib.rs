pub mod devices;

pub mod smart_home {
    use crate::devices::{DeviceBehavior, DeviceInfoProvider, DeviceState, Devices};

    pub struct SmartHouse {
        pub name: String,
        rooms: Vec<Room>,
    }

    impl SmartHouse {
        pub fn new(name: &str) -> Self {
            SmartHouse {
                name: name.to_string(),
                rooms: Vec::new(),
            }
        }

        pub fn add_room(&mut self, name: &str) {
            self.rooms.push(Room {
                name: name.to_string(),
                devices: Vec::new(),
            });
        }

        pub fn remove_room(&mut self, name: &str) {
            self.rooms.retain(|room| room.name != name);
        }

        pub fn get_rooms(&self) -> Vec<&str> {
            self.rooms.iter().map(|room| room.name.as_str()).collect()
        }

        pub fn devices(&self, room: &str) -> Option<Vec<Devices>> {
            self.rooms
                .iter()
                .find(|r| r.name == room)
                .map(|r| r.devices.clone())
        }

        pub fn add_device(&mut self, room: &str, device: Devices) {
            if let Some(room) = self.rooms.iter_mut().find(|r| r.name == room) {
                room.devices.push(device);
            }
        }

        pub fn remove_device(&mut self, room: &str, device: Devices) {
            if let Some(room) = self.rooms.iter_mut().find(|r| r.name == room) {
                room.devices.retain(|d| *d != device);
            }
        }

        pub fn create_report<T>(&self, info_provider: &T) -> String
        where
            T: DeviceInfoProvider,
        {
            let mut report = String::new();
            report += &format!("Report for house: {}\n", self.name);
            report += &format!("Rooms total: {}\n", self.rooms.len());

            for room in &self.rooms {
                report += &format!("Room: {}\n", room.name);
                let devices_list = room
                    .devices
                    .iter()
                    .map(|device| format!("{}", device))
                    .collect::<Vec<String>>()
                    .join(", ");
                report += &format!("Devices: {}\n", devices_list);
                for device in &room.devices {
                    let behavior = DeviceBehavior::default(); // Replace this with the correct behavior
                    let state = DeviceState::default(); // Replace this with the correct state
                    let device_info =
                        info_provider.device_info(&room.name, device, &behavior, &state);
                    match device_info {
                        Some(info) => report += &format!(" - {}: {}\n", device, info),
                        None => report += &format!(" - {}: Device not found\n", device),
                    }
                }
                report += "\n";
            }
            report
        }
    }

    struct Room {
        name: String,
        devices: Vec<Devices>,
    }
}
