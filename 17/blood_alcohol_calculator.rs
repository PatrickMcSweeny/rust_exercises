use std::io;
use std::io::Write;
use std::str::FromStr;

fn main() {
    let mut weight_input = String::new();
    let mut gender = String::new();
    let mut number_of_drinks_input = String::new();
    let mut ounces_per_drink_input = String::new();
    let mut abv_input = String::new();
    let mut hours_input = String::new();

    println!("Enter your weight");
    io::stdout().flush().expect("");

    io::stdin()
        .read_line(&mut weight_input)
        .expect("Failed to read input.");

    let weight = f32::from_str(weight_input.trim()).expect("Please enter a valid age.");

    println!("Enter your drinking gender");
    io::stdout().flush().expect("");

    io::stdin()
        .read_line(&mut gender)
        .expect("Failed to read input.");

    gender = String::from(gender.trim().to_lowercase());

    let ratio = match gender.as_str() {
        "m" => 0.73,
        "f" => 0.66,
        _ => return,
    };

    println!("Enter the number of drinks");
    io::stdout().flush().expect("");

    io::stdin()
        .read_line(&mut number_of_drinks_input)
        .expect("Failed to read input.");

    let number_of_drinks =
        f32::from_str(number_of_drinks_input.trim()).expect("Please enter a valid age.");

    println!("How many ounces per drink?");
    io::stdout().flush().expect("");

    io::stdin()
        .read_line(&mut ounces_per_drink_input)
        .expect("Failed to read input.");

    println!("Enter the alcohol by volume of the drinks");
    io::stdout().flush().expect("");

    io::stdin()
        .read_line(&mut abv_input)
        .expect("Failed to read input.");

    let abv = f32::from_str(abv_input.trim()).expect("Please enter a valid age.");

    let ounces_per_drink =
        f32::from_str(ounces_per_drink_input.trim()).expect("Please enter a valid age.");

    println!("How many hours since your last drink?");
    io::stdout().flush().expect("");

    io::stdin()
        .read_line(&mut hours_input)
        .expect("Failed to read input.");

    let hours = f32::from_str(hours_input.trim()).expect("Please enter a valid age.");

    let total_alcohol = (abv / 100.0) * ounces_per_drink * number_of_drinks;

    let blood_acohol_content = ((total_alcohol * 5.14) / (weight * ratio)) - 0.015 * hours;

    println!("Your BAC is {}", blood_acohol_content);

    let legal = if blood_acohol_content < 0.08 {
        "legal"
    } else {
        "not legal"
    };

    println!("It is {} for you to drive", legal);
}
