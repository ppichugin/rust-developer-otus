struct SmartSocket {
    description: String,
    is_on: bool,
    power_consumption: f32,
}

impl SmartSocket {
    fn new(description: String) -> SmartSocket {
        SmartSocket {
            description,
            is_on: false,
            power_consumption: 0.0,
        }
    }

    fn turn_on(&mut self) {
        self.is_on = true;
    }

    fn turn_off(&mut self) {
        self.is_on = false;
    }

    fn get_power_consumption(&self) -> f32 {
        self.power_consumption
    }
}

struct Thermometer {
    temperature: f32,
}

impl Thermometer {
    fn new() -> Thermometer {
        Thermometer { temperature: 0.0 }
    }

    fn get_temperature(&self) -> f32 {
        self.temperature
    }
}

fn main() {
    let mut socket = SmartSocket::new(String::from("Smart Socket 1"));
    let thermometer = Thermometer::new();

    socket.turn_on();
    println!("{} is on", socket.description);

    socket.power_consumption = 50.0;
    println!("Power consumption: {} W", socket.get_power_consumption());

    socket.turn_off();
    println!("{} is off", socket.description);

    let temperature = thermometer.get_temperature();
    println!("Current temperature: {}°C", temperature);
}