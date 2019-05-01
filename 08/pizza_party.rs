use std::io;
use std::io::Write;
use std::str::FromStr;

fn main() {
    let mut people_input = String::new();
    let mut pizzas_input = String::new();

    println!("How many people?");
    io::stdout().flush().expect("");

    io::stdin()
        .read_line(&mut people_input)
        .expect("Failed to read line");

    let num_people = u8::from_str(people_input.trim()).expect("Please enter a valid number");

    println!("How many pizzas do you have?");
    io::stdout().flush().expect("");

    io::stdin()
        .read_line(&mut pizzas_input)
        .expect("Failed to read line");

    let num_pizzas = u8::from_str(pizzas_input.trim()).expect("Please enter a valid number");
    let total_slices = num_pizzas * 8;

    println!("{} people with {} pizzas", num_people, num_pizzas);
    println!(
        "Each person gets {:.0} slices of pizza.",
        total_slices / num_people
    );
    println!("There are {} leftover slices.", total_slices % num_people);
}
