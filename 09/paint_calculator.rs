use std::io;
use std::io::Write;
use std::str::FromStr;

const SQUARE_FEET_PER_GALLON: f32 = 350.0;

fn main() {
    let mut length_input = String::new();
    let mut width_input = String::new();

    println!("How many feet is the length?");
    io::stdout().flush().expect("");

    io::stdin()
        .read_line(&mut length_input)
        .expect("Failed to read input.");

    let length = u16::from_str(length_input.trim()).expect("Please enter a valid number.");

    println!("How many feet is the width?");
    io::stdout().flush().expect("");

    io::stdin()
        .read_line(&mut width_input)
        .expect("Failed to read input.");

    let width = u16::from_str(width_input.trim()).expect("Please enter a valid number.");

    let square_feet: f32 = (length * width).into();
    let gallons: f32 = (square_feet / SQUARE_FEET_PER_GALLON).into();
    let rounded_gallons = gallons.ceil();

    let mut gallon_string = String::from("gallon");
    if rounded_gallons > 1.0 {
        gallon_string.push('s');
    }

    println!(
        "You will need to purchase {} {} of paint to to cover {} square feet.",
        rounded_gallons, gallon_string, square_feet
    );
}
