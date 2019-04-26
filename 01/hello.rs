use std::io;
use std::io::Write;

fn main() {
    print!("What is your name? ");
    io::stdout().flush().expect("");

    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read name.");

    println!("Hello, {}, nice to meet you!", name.trim());
}
