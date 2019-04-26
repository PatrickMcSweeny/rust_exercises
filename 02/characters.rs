use std::io;
use std::io::Write;

fn main() {
    print!("What is the input string? ");
    io::stdout().flush().expect("");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input = input.trim().to_string();

    println!("{} has {} characters", input, input.len());
}
