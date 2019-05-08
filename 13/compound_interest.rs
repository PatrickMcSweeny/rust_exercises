use std::io;
use std::io::Write;
use std::str::FromStr;

fn main() {
    let mut principal_input = String::new();
    let mut rate_input = String::new();
    let mut years_input = String::new();
    let mut periods_input = String::new();

    println!("What is the principal amount?");
    io::stdout().flush().expect("");

    io::stdin()
        .read_line(&mut principal_input)
        .expect("Failed to read input.");

    let principal = f32::from_str(principal_input.trim()).expect("Please enter a valid number.");
    io::stdout().flush().expect("");

    println!("What is the rate?");

    io::stdin()
        .read_line(&mut rate_input)
        .expect("Failed to read input.");

    let rate = f32::from_str(rate_input.trim()).expect("Please enter a valid number");

    println!("What is the number of years?");
    io::stdout().flush().expect("");

    io::stdin()
        .read_line(&mut years_input)
        .expect("Failed to read input.");

    let years = u16::from_str(years_input.trim()).expect("Please enter a valid number");

    println!("What is the number of times the interest is compounded per year?");
    io::stdout().flush().expect("");

    io::stdin()
        .read_line(&mut periods_input)
        .expect("Failed to read input.");

    let times_per_year = u16::from_str(periods_input.trim()).expect("Please enter a valid number");

    let actual_rate: f32 = rate / 100.0;
    let total: f32 = principal * (1.0 + (actual_rate / times_per_year as f32))
        .powi((times_per_year * years) as i32);

    println!("{:.2} invested at {}% for {} years", principal, rate, years);
    println!("compounded {:?} per year is ${:.2}", times_per_year, total);
}
