use std::io;
use std::io::Write;
use std::str::FromStr;

fn main() {
    let mut items = Vec::new();

    for i in 1..3 {
        let mut price_input = String::new();
        let mut quantity_input = String::new();
        let mut item = Vec::new();
        println!("Enter the price of item {}", i);
        io::stdout().flush().expect("");

        io::stdin()
            .read_line(&mut price_input)
            .expect("Failed to read input.");

        item.push(f32::from_str(price_input.trim()).expect("Please enter a valid number."));

        println!("Enter the quantity of item {}", i);
        io::stdout().flush().expect("");

        io::stdin()
            .read_line(&mut quantity_input)
            .expect("Failed to read input.");

        item.push(
            u8::from_str(quantity_input.trim()).expect("Please enter a valid number.") as f32,
        );

        items.push(item);
    }

    let subtotal: f32 = items.iter().map(|item| item[0] * item[1]).sum();
    let tax: f32 = subtotal * 0.055;
    let total: f32 = subtotal + tax;

    println!("Subtotal: ${:.2}", subtotal);
    println!("Tax: ${:.2}", tax);
    println!("Total: ${:.2}", total);
}
