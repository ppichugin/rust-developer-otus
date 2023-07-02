use smart_house::devices::{
    BorrowingDeviceInfoProvider, OwningDeviceInfoProvider, SmartSocket, SmartThermometer,
};
use smart_house::smart_home::SmartHouse;

fn main() {
    let socket1 = SmartSocket {};
    let socket2 = SmartSocket {};
    let thermo = SmartThermometer {};

    let house = SmartHouse::new("My House");

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
