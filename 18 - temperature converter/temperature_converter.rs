use std::io;
use std::io::Write;
use std::str::FromStr;

fn main() {
    let mut choice = String::new();
    let mut input_temp = String::new();

    println!("Press f to convert from farenheit to celsius");
    println!("Press c to convert from celsius to farenheit");
    println!("Your choice:");
    io::stdout().flush().expect("");

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read input.");

    choice = choice.trim().to_lowercase();

    let output = match choice.as_str() {
        "f" => ("farenheit", "celsius"),
        "c" => ("celsius", "farenheit"),
        _ => return,
    };

    println!("Enter the temperature in {}", output.0);
    io::stdout().flush().expect("");

    io::stdin()
        .read_line(&mut input_temp)
        .expect("Failed to read input.");

    let temp_to_convert = f32::from_str(input_temp.trim()).expect("Please enter a valid number");

    let output_temp = match choice.as_str() {
        "f" => (temp_to_convert - 32.0) * 5.0 / 9.0,
        "c" => (temp_to_convert * 9.0 / 5.0) + 32.0,
        _ => return,
    };

    println!("The temperature in {} is {}", output.1, output_temp);
}
