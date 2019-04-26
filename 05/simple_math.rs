use std::io;
use std::io::Write;
use std::process;

fn main() {
    let mut first_number = String::new();
    let mut second_number = String::new();
    let mut parsed_numbers: Vec<f64> = Vec::new();

    print!("What is the first number? ");
    flush_io();

    io::stdin()
        .read_line(&mut first_number)
        .expect("Failed to read input");

    let result = first_number.trim().parse::<f64>();

    match result {
        Ok(_ok) => parsed_numbers.push(first_number.trim().parse::<f64>().unwrap()),
        Err(_e) => input_error(),
    }

    print!("What is the second number? ");
    flush_io();

    io::stdin()
        .read_line(&mut second_number)
        .expect("Failed to read input");

    let result = second_number.trim().parse::<f64>();

    match result {
        Ok(_ok) => parsed_numbers.push(second_number.trim().parse::<f64>().unwrap()),
        Err(_e) => input_error(),
    }

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

fn input_error() {
    println!("Invalid number");
    process::exit(0);
}
