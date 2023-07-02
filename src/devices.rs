pub trait DeviceInfoProvider {
    fn device_info(&self, room: &str, device: &Devices) -> Option<String>;
}

#[derive(Debug, Clone, PartialEq)]
pub enum Devices {
    TV,
    Lights,
    Oven,
    Microwave,
    Thermometer,
}

pub struct SmartSocket {}

impl DeviceInfoProvider for SmartSocket {
    fn device_info(&self, _room: &str, device: &Devices) -> Option<String> {
        match device {
            Devices::TV => Some("Power: On".to_string()),
            Devices::Lights => Some("Brightness: 80%".to_string()),
            Devices::Oven => Some("Power: 2000W".to_string()),
            Devices::Microwave => Some("Power: 800W".to_string()),
            _ => None,
        }
    }
}

pub struct SmartThermometer {}

impl DeviceInfoProvider for SmartThermometer {
    fn device_info(&self, _room: &str, device: &Devices) -> Option<String> {
        match device {
            Devices::Thermometer => Some("Temperature: 20C".to_string()),
            _ => None,
        }
    }
}

pub struct OwningDeviceInfoProvider {
    pub socket: SmartSocket,
}

impl DeviceInfoProvider for OwningDeviceInfoProvider {
    fn device_info(&self, room: &str, device: &Devices) -> Option<String> {
        match device {
            Devices::TV | Devices::Lights | Devices::Oven | Devices::Microwave => {
                self.socket.device_info(room, device)
            }
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
            Devices::TV | Devices::Lights | Devices::Oven | Devices::Microwave => {
                self.socket.device_info(room, device)
            }
            Devices::Thermometer => self.thermo.device_info(room, device),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smart_socket_device_info() {
        let socket = SmartSocket {};
        assert_eq!(
            socket.device_info("Living Room", &Devices::TV),
            Some("Power: On".to_string())
        );
        assert_eq!(
            socket.device_info("Bedroom", &Devices::Lights),
            Some("Brightness: 80%".to_string())
        );
        assert_eq!(
            socket.device_info("Kitchen", &Devices::Oven),
            Some("Power: 2000W".to_string())
        );
        assert_eq!(
            socket.device_info("Kitchen", &Devices::Microwave),
            Some("Power: 800W".to_string())
        );
        assert_eq!(socket.device_info("Bathroom", &Devices::Thermometer), None);
    }

    #[test]
    fn test_smart_thermometer_device_info() {
        let thermometer = SmartThermometer {};
        assert_eq!(
            thermometer.device_info("Living Room", &Devices::Thermometer),
            Some("Temperature: 20C".to_string())
        );
        assert_eq!(thermometer.device_info("Kitchen", &Devices::TV), None);
        assert_eq!(thermometer.device_info("Bedroom", &Devices::Lights), None);
        assert_eq!(thermometer.device_info("Bathroom", &Devices::Oven), None);
    }

    #[test]
    fn test_owning_device_info_provider() {
        let socket = SmartSocket {};
        let provider = OwningDeviceInfoProvider { socket };
        assert_eq!(
            provider.device_info("Living Room", &Devices::TV),
            Some("Power: On".to_string())
        );
        assert_eq!(
            provider.device_info("Bedroom", &Devices::Lights),
            Some("Brightness: 80%".to_string())
        );
        assert_eq!(
            provider.device_info("Kitchen", &Devices::Oven),
            Some("Power: 2000W".to_string())
        );
        assert_eq!(
            provider.device_info("Kitchen", &Devices::Microwave),
            Some("Power: 800W".to_string())
        );
        assert_eq!(
            provider.device_info("Bathroom", &Devices::Thermometer),
            None
        );
    }

    #[test]
    fn test_borrowing_device_info_provider() {
        let socket = SmartSocket {};
        let thermometer = SmartThermometer {};
        let provider = BorrowingDeviceInfoProvider {
            socket: &socket,
            thermo: &thermometer,
        };
        assert_eq!(
            provider.device_info("Living Room", &Devices::TV),
            Some("Power: On".to_string())
        );
        assert_eq!(
            provider.device_info("Bedroom", &Devices::Lights),
            Some("Brightness: 80%".to_string())
        );
        assert_eq!(
            provider.device_info("Kitchen", &Devices::Oven),
            Some("Power: 2000W".to_string())
        );
        assert_eq!(
            provider.device_info("Kitchen", &Devices::Microwave),
            Some("Power: 800W".to_string())
        );
        assert_eq!(
            provider.device_info("Bathroom", &Devices::Thermometer),
            Some("Temperature: 20C".to_string())
        );
    }
}
