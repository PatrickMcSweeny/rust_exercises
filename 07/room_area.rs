use std::io;
use std::io::Write;
use std::str::FromStr;

const CONVERSION: f32 = 0.09290304;

fn main() {
    let mut length_input = String::new();
    let mut width_input = String::new();
    let mut dimensions: Vec<f32> = Vec::new();

    println!("What is the length of the room? ");
    flush_io();

    io::stdin()
        .read_line(&mut length_input)
        .expect("Failed to read input.");

    dimensions.push(f32::from_str(length_input.trim()).expect("Please enter a valid number."));

    println!("What is the width of the room? ");
    flush_io();

    io::stdin()
        .read_line(&mut width_input)
        .expect("Failed to read input.");

    dimensions.push(f32::from_str(width_input.trim()).expect("Please enter a valid number."));

    let square_feet = dimensions[0] * dimensions[1];

    println!(
        "You entered dimensions of {} feet by {} feet.",
        dimensions[0], dimensions[1]
    );
    println!("The area is");
    println!("{} square feet", square_feet);
    println!("{:.3} square meters", square_meters(square_feet));
}

fn square_meters(square_feet: f32) -> f32 {
    square_feet * CONVERSION
}

fn flush_io() {
    io::stdout().flush().expect("");
}
