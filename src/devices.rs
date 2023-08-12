use std::fmt;

pub trait DeviceInfoProvider {
    fn device_info(
        &self,
        room: &str,
        device: &Devices,
        behavior: &DeviceBehavior,
        state: &DeviceState,
    ) -> Option<String>;
}

#[derive(Debug, Clone, PartialEq)]
pub enum Devices {
    TV { power: bool },
    Lights { brightness: u8 },
    Oven { power: u16 },
    Microwave { power: u16 },
    Thermometer { temperature: f32 },
    Custom(String, DeviceBehavior, DeviceState),
}

// New enum to represent the state of each device
#[derive(Debug, Clone, PartialEq, Default)]
pub enum DeviceState {
    #[default]
    Off,
    On,
    Idle,
}

impl fmt::Display for DeviceState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DeviceState::Off => write!(f, "Off"),
            DeviceState::On => write!(f, "On"),
            DeviceState::Idle => write!(f, "Idle"),
        }
    }
}

// New enum to represent the behavior of each custom device
#[derive(Debug, Clone, PartialEq, Default)]
pub enum DeviceBehavior {
    #[default]
    Undefined,
    BrewingCoffee,
    BoilingWater,
}

pub fn register_device(name: &str, behavior: DeviceBehavior, state: DeviceState) -> Devices {
    Devices::Custom(name.to_string(), behavior, state)
}

impl fmt::Display for Devices {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Devices::TV { .. } => write!(f, "TV"),
            Devices::Lights { .. } => write!(f, "Lights"),
            Devices::Oven { .. } => write!(f, "Oven"),
            Devices::Microwave { .. } => write!(f, "Microwave"),
            Devices::Thermometer { .. } => write!(f, "Thermometer"),
            Devices::Custom(name, _, _) => write!(f, "{}", name),
        }
    }
}

pub struct SmartSocket {}

impl DeviceInfoProvider for SmartSocket {
    fn device_info(
        &self,
        _room: &str,
        device: &Devices,
        _behavior: &DeviceBehavior,
        _state: &DeviceState,
    ) -> Option<String> {
        match device {
            Devices::TV { .. } => Some("Power: On".to_string()),
            Devices::Lights { .. } => Some("Brightness: 80%".to_string()),
            Devices::Oven { .. } => Some("Power: 2000W".to_string()),
            Devices::Microwave { .. } => Some("Power: 800W".to_string()),
            _ => None,
        }
    }
}

pub struct SmartThermometer {}

impl DeviceInfoProvider for SmartThermometer {
    fn device_info(
        &self,
        _room: &str,
        device: &Devices,
        _behavior: &DeviceBehavior,
        _state: &DeviceState,
    ) -> Option<String> {
        match device {
            Devices::Thermometer { .. } => Some("Temperature: 20C".to_string()),
            _ => None,
        }
    }
}

pub struct OwningDeviceInfoProvider {
    pub socket: SmartSocket,
}

impl DeviceInfoProvider for OwningDeviceInfoProvider {
    fn device_info(
        &self,
        _room: &str,
        device: &Devices,
        _behavior: &DeviceBehavior,
        _state: &DeviceState,
    ) -> Option<String> {
        match device {
            Devices::TV { power } => Some(format!("Power: {}", if *power { "On" } else { "Off" })),
            Devices::Lights { brightness } => Some(format!("Brightness: {}%", brightness)),
            Devices::Oven { power } => Some(format!("Power: {}W", power)),
            Devices::Microwave { power } => Some(format!("Power: {}W", power)),
            Devices::Thermometer { temperature } => Some(format!("Temperature: {}C", temperature)),
            Devices::Custom(name, behavior, state) => {
                Some(format!("{} is {} and {:?}", name, state, behavior))
            }
        }
    }
}
