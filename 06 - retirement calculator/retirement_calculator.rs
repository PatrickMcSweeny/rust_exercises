extern crate chrono;
use chrono::prelude::*;
use std::io;
use std::io::Write;
use std::str::FromStr;

fn main() {
    let year = Utc::now().year() as u16;
    let mut age_input = String::new();
    let mut retirement_age_input = String::new();

    println!("What is your current age?");
    flush_io();

    io::stdin()
        .read_line(&mut age_input)
        .expect("Failed to read input");

    let age = u16::from_str(&age_input.trim()).expect("Please enter a valid number");

    println!("At what age would you like to retire?");
    flush_io();

    io::stdin()
        .read_line(&mut retirement_age_input)
        .expect("Failed to read input");

    let retirement_age =
        u16::from_str(&retirement_age_input.trim()).expect("Please enter a valid number");

    let years_left = retirement_age - age;

    println!("You have {} years left until you can retire.", years_left);
    println!("It's {}, so you can retire in {}.", year, year + years_left);
}

fn flush_io() {
    io::stdout().flush().expect("");
}
