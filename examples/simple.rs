use smart_house::devices::{
    BorrowingDeviceInfoProvider, DeviceBehavior, DeviceState, Devices, SmartSocket,
    SmartThermometer,
};
use smart_house::smart_home::SmartHouse;

fn main() {
    let socket = SmartSocket {};
    let thermometer = SmartThermometer {};

    let mut house = SmartHouse::new("My House");

    // Add rooms to the house
    house.add_room("Living Room");
    house.add_room("Kitchen");

    // Add existing devices to rooms
    house.add_device("Living Room", Devices::TV { power: true });
    house.add_device("Living Room", Devices::Lights { brightness: 80 });
    house.add_device("Kitchen", Devices::Oven { power: 2000 });
    house.add_device("Kitchen", Devices::Microwave { power: 800 });

    // Register a new custom device
    let custom_device = Devices::Custom(
        "Coffee-machine".to_string(),
        DeviceBehavior::BoilingWater,
        DeviceState::On,
    );

    // Add the custom device to a room
    house.add_device("Living Room", custom_device);

    let info_provider = BorrowingDeviceInfoProvider {
        socket: &socket,
        thermo: &thermometer,
    };
    let report = house.create_report(&info_provider);

    println!("Report:\n{}", report);
}
