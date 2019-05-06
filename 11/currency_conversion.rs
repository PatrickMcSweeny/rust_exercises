use std::io;
use std::io::Write;
use std::str::FromStr;

fn main() {
    let mut amount_from_input = String::new();
    let mut exchange_rate_input = String::new();

    println!("How many euros are you exchanging?");
    io::stdout().flush().expect("");

    io::stdin()
        .read_line(&mut amount_from_input)
        .expect("Failed to read input.");

    let euros = f32::from_str(amount_from_input.trim()).expect("Please enter a valid number.");

    println!("What is the exchange rate?");
    io::stdout().flush().expect("");

    io::stdin()
        .read_line(&mut exchange_rate_input)
        .expect("Failed to read input.");

    let exchange_rate =
        f32::from_str(exchange_rate_input.trim()).expect("Please enter a valid number.");

    let dollars: f32 = euros * (exchange_rate / 100.0);

    println!(
        "{} euros at an exchange rate of {:.2} is\n{:.2} U.S. dollars",
        euros, exchange_rate, dollars
    );
}
