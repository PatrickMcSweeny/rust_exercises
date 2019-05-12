use std::io;
use std::io::Write;
use std::str::FromStr;

fn main() {
    let mut order_amount_input = String::new();
    let mut state = String::new();
    let mut output = String::new();

    println!("What is the order amount?");
    io::stdout().flush().expect("");

    io::stdin()
        .read_line(&mut order_amount_input)
        .expect("Failed to read input.");

    let mut order_total =
        f32::from_str(order_amount_input.trim()).expect("Please enter a valid number.");

    println!("What state will it be shipped to?");
    io::stdout().flush().expect("");

    io::stdin()
        .read_line(&mut state)
        .expect("Failed to read input.");

    state = state.trim().to_uppercase();

    let tax: f32 = match state.as_str() {
        "WI" => wisconsin_tax(order_total),
        "IL" => order_total * 0.08,
        _ => 0.0,
    };

    if tax > 0.0 {
        output.push_str(&format!("The tax is ${:.2}\n", tax));
        order_total += tax;
    }

    output.push_str(&format!("The total is ${:.2}", order_total));

    println!("{}", output);
}

fn wisconsin_tax(amount: f32) -> f32 {
    let state_tax_rate: f32 = 0.055;
    let mut county = String::new();

    println!("What county will it be shipped to?");
    io::stdout().flush().expect("");

    io::stdin()
        .read_line(&mut county)
        .expect("Failed to read input.");

    county = county.trim().to_lowercase();

    let county_tax_rate = match county.as_str() {
        "eau claire" => 0.005,
        "dunn" => 0.004,
        _ => 0.0,
    };

    amount * county_tax_rate + amount * state_tax_rate
}
