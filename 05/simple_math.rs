use std::io;
use std::io::Write;
use std::str::FromStr;

fn main() {
    let mut first_number = String::new();
    let mut second_number = String::new();
    let mut parsed_numbers: Vec<f64> = Vec::new();

    print!("What is the first number? ");
    flush_io();

    io::stdin()
        .read_line(&mut first_number)
        .expect("Failed to read input");

    parsed_numbers.push(f64::from_str(&first_number.trim()).expect("Please enter a valid number."));

    print!("What is the second number? ");
    flush_io();

    io::stdin()
        .read_line(&mut second_number)
        .expect("Failed to read input");

    parsed_numbers
        .push(f64::from_str(&second_number.trim()).expect("Please enter a valid number."));

    println!(
        "{} + {} = {}",
        parsed_numbers[0],
        parsed_numbers[1],
        parsed_numbers[0] + parsed_numbers[1]
    );
    println!(
        "{} - {} = {}",
        parsed_numbers[0],
        parsed_numbers[1],
        parsed_numbers[0] - parsed_numbers[1]
    );
    println!(
        "{} * {} = {}",
        parsed_numbers[0],
        parsed_numbers[1],
        parsed_numbers[0] * parsed_numbers[1]
    );
    println!(
        "{} / {} = {}",
        parsed_numbers[0],
        parsed_numbers[1],
        parsed_numbers[0] / parsed_numbers[1]
    );
}

fn flush_io() {
    io::stdout().flush().expect("");
}
