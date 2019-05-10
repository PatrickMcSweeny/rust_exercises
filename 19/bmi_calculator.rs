use std::io;
use std::io::Write;
use std::str::FromStr;

fn main() {
    let mut height_input = String::new();
    let mut weight_input = String::new();

    println!("Enter your height in inches");
    io::stdout().flush().expect("");

    io::stdin()
        .read_line(&mut height_input)
        .expect("Failed to read input.");

    let height = f32::from_str(height_input.trim()).expect("Please enter a valid number.");

    println!("Enter your weight");
    io::stdout().flush().expect("");

    io::stdin()
        .read_line(&mut weight_input)
        .expect("Failed to read input.");

    let weight = f32::from_str(weight_input.trim()).expect("Please enter a valid number.");

    let bmi = (weight / (height * height)) * 703.0;

    println!("Your BMI is {:.1}", bmi);

    let output = match bmi {
        b if b < 18.5 => "You are underweight. Please see your doctor.",
        b if b > 25.0 => "You are overweight. Please see your doctor.",
        _ => "You are within the ideal weight range",
    };

    println!("{}", output);
}
