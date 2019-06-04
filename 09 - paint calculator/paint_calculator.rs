use std::io;
use std::io::Write;
use std::str::FromStr;

const SQUARE_FEET_PER_GALLON: u16 = 350;

fn main() {
    let mut length_input = String::new();
    let mut width_input = String::new();

    println!("How many feet is the length?");
    io::stdout().flush().expect("");

    io::stdin()
        .read_line(&mut length_input)
        .expect("Failed to read input.");

    let length = f32::from_str(length_input.trim()).expect("Please enter a valid number.");

    println!("How many feet is the width?");
    io::stdout().flush().expect("");

    io::stdin()
        .read_line(&mut width_input)
        .expect("Failed to read input.");

    let width = f32::from_str(width_input.trim()).expect("Please enter a valid number.");

    let square_feet: f32 = length * width;
    let gallons: f32 = (square_feet / SQUARE_FEET_PER_GALLON as f32).into();
    let rounded_gallons = gallons.ceil();

    let gallon_string = if rounded_gallons > 1.0 {
        String::from("gallons")
    }else{
        String::from("gallon")
    };

    println!(
        "You will need to purchase {} {} of paint to to cover {} square feet.",
        rounded_gallons, gallon_string, square_feet
    );
}
