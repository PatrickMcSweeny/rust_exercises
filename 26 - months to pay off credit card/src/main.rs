use std::io;
use std::io::Write;
use std::str::FromStr;

fn main() {
    let balance = get_input("What is your balance?");
    let apr = get_input("What is the APR on the card (as a percent)?");
    let monthly_payment = get_input("What is the monthly payment you can make?");

    let months = calculate_months_until_paid_off(balance, apr, monthly_payment);

    println!("It will take you {} months to pay off this card.", months);
}

fn calculate_months_until_paid_off(balance: f32, apr: f32, monthly_payment: f32) -> u8 {
    let daily_rate: f32 = (apr / 100.0) / 365.0;
    (-(1.0 / 30.0) * (1.0 + balance / monthly_payment * (1.0 - (1.0 + daily_rate).powi(30))).log2()
        / (1.0 + daily_rate).log2())
    .ceil() as u8
}

fn get_input(output: &str) -> f32 {
    print!("{} ", output);

    let mut input = String::new();
    io::stdout().flush().expect("");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    f32::from_str(input.trim()).expect("Please enter a valid number.")
}

#[test]
fn test_calculate_months_until_paid_off() {
    assert_eq!(70, calculate_months_until_paid_off(5000.0, 12.0, 100.0));
}
