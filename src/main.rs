use smart_house::devices;
use smart_house::devices::{
    BorrowingDeviceInfoProvider, DeviceBehavior, DeviceState, Devices, OwningDeviceInfoProvider,
    SmartSocket, SmartThermometer,
};
use smart_house::smart_home::SmartHouse;

fn main() {
    let socket1 = SmartSocket {};
    let socket2 = SmartSocket {};
    let thermo = SmartThermometer {};

    let mut house = SmartHouse::new("My House");

    // Add rooms to the house
    let _ = house.add_room("Living Room");
    let _ = house.add_room("Kitchen");

    // Add the same room twice
    if let Err(e) = house.add_room("Living Room") {
        println!("Error: {}\n", e);
    }

    // Add existing devices to rooms
    house.add_device("Living Room", Devices::TV { power: true });
    house.add_device("Living Room", Devices::Lights { brightness: 80 });
    house.add_device("Kitchen", Devices::Oven { power: 2000 });
    house.add_device("Kitchen", Devices::Microwave { power: 800 });

    // Remove room twice with devices
    if let Err(e) = house.remove_room("Living Room") {
        println!("Error: {}\n", e);
    }

    // Register a new custom device
    let custom_device = devices::register_device(
        "Coffee-machine",
        DeviceBehavior::BrewingCoffee,
        DeviceState::On,
    );

    // Add the custom device to a room
    house.add_device("Living Room", custom_device);

    let info_provider_1 = OwningDeviceInfoProvider { socket: socket1 };
    let report1 = house.create_report(&info_provider_1);

    let info_provider_2 = BorrowingDeviceInfoProvider {
        socket: &socket2,
        thermo: &thermo,
    };
    let report2 = house.create_report(&info_provider_2);

    println!("Report #1:\n{}", report1);
    println!("Report #2:\n{}", report2);
}
