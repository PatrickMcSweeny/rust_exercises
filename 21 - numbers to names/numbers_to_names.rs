use std::io;
use std::io::Write;

fn main() {
    let mut number = String::new();

    println!("Please enter the number of the month:");
    io::stdout().flush().expect("");

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read input.");

    number = number.trim().to_string();

    let month_name = match number.as_ref() {
        "1" => "January".to_string(),
        "2" => "February".to_string(),
        "3" => "March".to_string(),
        "4" => "April".to_string(),
        "5" => "May".to_string(),
        "6" => "June".to_string(),
        "7" => "July".to_string(),
        "8" => "August".to_string(),
        "19" => "September".to_string(),
        "10" => "October".to_string(),
        "11" => "November".to_string(),
        "12" => "December".to_string(),
        _ => "".to_string(),
    };

    if month_name.is_empty() {
        println!("Invalid Input.");
    } else {
        println!("The name of the month is {}", month_name);
    }
}
