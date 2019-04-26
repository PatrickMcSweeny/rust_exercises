use std::io;
use std::io::Write;

fn main() {
    let mut quote = String::new();
    let mut quotee = String::new();

    print!("What is the quote? ");
    flushio();

    io::stdin()
        .read_line(&mut quote)
        .expect("Failed to read input");

    quote = quote.trim().to_string();

    print!("Who said it? ");
    flushio();

    io::stdin()
        .read_line(&mut quotee)
        .expect("Failed to read input");
    quotee = quotee.trim().to_string();

    println!("{} says {}", quotee, &quote);
}

fn flushio(){
    io::stdout().flush().expect("");
}
