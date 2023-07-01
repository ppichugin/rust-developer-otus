use rust_developer::device_providers::{SmartSocket, SmartThermometer};
use rust_developer::smart_home::{DeviceInfoProvider, SmartHouse};

fn main() {
    let socket1 = SmartSocket {};
    let socket2 = SmartSocket {};
    let thermo = SmartThermometer {};

    let house = SmartHouse::new();

    let info_provider_1: Box<dyn DeviceInfoProvider> = Box::new(socket1);
    let report1 = house.create_report(&*info_provider_1);

    let info_provider_2: Box<dyn DeviceInfoProvider> = Box::new(socket2);
    let report2 = house.create_report(&*info_provider_2);

    let info_provider_3 = &thermo as &dyn DeviceInfoProvider;
    let report3 = house.create_report(info_provider_3);

    println!("Report #1:\n{}", report1);
    println!("Report #2:\n{}", report2);
    println!("Report #3:\n{}", report3);
}
