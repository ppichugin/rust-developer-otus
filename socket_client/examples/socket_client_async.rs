use std::io;

use socket_client::{Command, SmartSocketClient};

#[tokio::main]
async fn main() {
    let mut client = SmartSocketClient::new("127.0.0.1:7890").await.unwrap();

    loop {
        show_menu();
        let input = read_input();
        std::process::Command::new("clear").status().unwrap();

        let response = match input {
            Some(command) => client.run_command(command).await.unwrap(),
            None => {
                println!("Bye...");
                break;
            }
        };

        println!("Response: {response}");
    }
}

fn show_menu() {
    println!("\n------------------");
    println!("Select action:");
    println!("1) turn off");
    println!("2) turn on");
    println!("3) status");
    println!("4) power level");
    println!("_) exit");
}

fn read_input() -> Option<Command> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let cmd = match input.trim() {
        "1" => Command::TurnOff,
        "2" => Command::TurnOn,
        "3" => Command::GetStatus,
        "4" => Command::GetPower,
        _ => return None,
    };

    Some(cmd)
}
