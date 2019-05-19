use std::io;
use std::io::Write;
use std::str::FromStr;

fn main() {
    let mut number_input = String::new();
    let mut numbers: Vec<f64> = Vec::new();
    let number_names = vec!["first", "second", "third"];

    for number_name in &number_names {
        println!("Enter the {} number:", number_name);
        io::stdout().flush().expect("");

        io::stdin()
            .read_line(&mut number_input)
            .expect("Failed to read input.");

        numbers.push(f64::from_str(number_input.trim()).expect("Please enter a valid number"));
        number_input.clear();
    }

    let largest_number: f64 = find_largest_number(numbers);

    println!("The largest number is {}", largest_number);
}

fn find_largest_number(numbers: Vec<f64>) -> f64 {
    if numbers[0] > numbers[1] && numbers[0] > numbers[2] {
        numbers[0]
    } else if numbers[1] > numbers[2] {
        numbers[1]
    } else {
        numbers[2]
    }
}
