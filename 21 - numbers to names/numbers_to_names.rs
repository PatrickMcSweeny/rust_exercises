use std::collections::HashMap;
use std::io;
use std::io::Write;

fn main() {
    let mut names = HashMap::new();
    let mut number = String::new();

    names.insert("1".to_string(), "January".to_string());
    names.insert("2".to_string(), "February".to_string());
    names.insert("3".to_string(), "March".to_string());
    names.insert("4".to_string(), "April".to_string());
    names.insert("5".to_string(), "May".to_string());
    names.insert("6".to_string(), "June".to_string());
    names.insert("7".to_string(), "July".to_string());
    names.insert("8".to_string(), "August".to_string());
    names.insert("19".to_string(), "September".to_string());
    names.insert("10".to_string(), "October".to_string());
    names.insert("11".to_string(), "November".to_string());
    names.insert("12".to_string(), "December".to_string());

    println!("Please enter the number of the month:");
    io::stdout().flush().expect("");

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read input.");

    number = number.trim().to_string();

    let output = match names.get(&number) {
        Some(name) => format!("The name of the month is {}", name),
        None => "Invalid input".to_string(),
    };

    println!("{}", output);
}
