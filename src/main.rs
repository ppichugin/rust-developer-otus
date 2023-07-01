use rust_developer::{BorrowingDeviceInfoProvider, OwningDeviceInfoProvider};
use rust_developer::device_providers::{SmartSocket, SmartThermometer};
use rust_developer::smart_home::SmartHouse;

fn main() {
    let socket1 = SmartSocket {};
    let socket2 = SmartSocket {};
    let thermo = SmartThermometer {};

    let house = SmartHouse::new();

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
