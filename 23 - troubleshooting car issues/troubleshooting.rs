use std::io;
use std::io::Write;

fn main() {
    car_silent();
}

fn car_silent() {
    let input = get_input("Is the car silent when you turn the key?".to_string());

    match input.as_str() {
        "y" => check_terminals(),
        "n" => check_noise(),
        _ => println!("Invalid input"),
    }
}

fn check_terminals() {
    let input = get_input("Are the terminals corroded?".to_string());

    match input.as_str() {
        "y" => println!("Clean terminals and try again."),
        "n" => println!("Replace cables and try again."),
        _ => println!("Invalid input"),
    }
}

fn check_noise() {
    let input = get_input("Does the car make a clicking noise?".to_string());

    match input.as_str() {
        "y" => println!("Replace the battery."),
        "n" => check_crank_up(),
        _ => println!("Invalid input"),
    }
}

fn check_crank_up() {
    let input = get_input("Does the car crank up but fail to start?".to_string());

    match input.as_str() {
        "y" => println!("Check spark plug connections."),
        "n" => check_fuel_injection(),
        _ => println!("Invalid input"),
    }
}

fn check_fuel_injection() {
    get_input("Does the engine start and then die?".to_string());
    // The exercise doesn't cover a "no" answer for the above question

    let input = get_input("Does the car have fuel injection?".to_string());

    match input.as_str() {
        "y" => println!("Get it in for service."),
        "n" => println!("Check to ensure the choke is opening and closing"),
        _ => println!("Invalid input"),
    }
}

fn get_input(output: String) -> String {
    println!("{}", output);
    io::stdout().flush().expect("");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_lowercase()
}
