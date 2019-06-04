use std::io;
use std::io::Write;
use std::str::FromStr;

fn main() {
    let mut order_amount_input = String::new();
    let mut state = String::new();
    let mut output = String::new();
    let wisconsin_tax: f32 = 0.055;

    println!("What is the order amount?");
    io::stdout().flush().expect("");

    io::stdin()
        .read_line(&mut order_amount_input)
        .expect("Failed to read input.");

    let mut total = f32::from_str(order_amount_input.trim()).expect("Please enter a valid number.");

    println!("What is the state?");
    io::stdout().flush().expect("");

    io::stdin()
        .read_line(&mut state)
        .expect("Failed to read input.");

    state = state.trim().to_uppercase();

    if state == "WI" {
        output.push_str(&format!("The subtotal is ${:.2}\n", total));
        let tax = total * wisconsin_tax;
        total += tax;
        output.push_str(&format!("The tax is ${:.2}\n", tax));
    }

    output.push_str(&format!("The total is ${:.2}", total));

    println!("{}", output);
}
