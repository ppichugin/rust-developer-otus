use smart_house::devices::{BorrowingDeviceInfoProvider, SmartSocket, SmartThermometer};
use smart_house::smart_home::SmartHouse;

fn main() {
    let socket = SmartSocket {};
    let thermometer = SmartThermometer {};

    let house = SmartHouse::new("My House");

    let info_provider = BorrowingDeviceInfoProvider {
        socket: &socket,
        thermo: &thermometer,
    };
    let report = house.create_report(&info_provider);

    println!("Report:\n{}", report);
}
