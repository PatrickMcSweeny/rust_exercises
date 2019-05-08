use std::io;
use std::io::Write;
use std::str::FromStr;

fn main() {
    let mut principal_input = String::new();
    let mut rate_input = String::new();
    let mut years_input = String::new();

    println!("Enter the principal:");
    io::stdout().flush().expect("");

    io::stdin()
        .read_line(&mut principal_input)
        .expect("Failed to read input.");

    let principal = f32::from_str(principal_input.trim()).expect("Please enter a valid number.");
    io::stdout().flush().expect("");

    println!("Enter the rate of interest:");

    io::stdin()
        .read_line(&mut rate_input)
        .expect("Failed to read input.");

    let rate = f32::from_str(rate_input.trim()).expect("Please enter a valid number");

    println!("Enter the number of years:");
    io::stdout().flush().expect("");

    io::stdin()
        .read_line(&mut years_input)
        .expect("Failed to read input.");

    let years = u16::from_str(years_input.trim()).expect("Please enter a valid number");

    let total: f32 = principal * (1.0 + (rate / 100.0) * years as f32);

    println!("After {} years at {}%,", years, rate);
    println!("the invesment will be worth ${:.2}", total);
}
