use std::io;
use std::io::Write;

fn main() {
    let mut noun = String::new();
    let mut verb = String::new();
    let mut adjective = String::new();
    let mut adverb = String::new();

    print!("Enter a noun: ");
    flushio();

    io::stdin()
        .read_line(&mut noun)
        .expect("Failed to read input");

    print!("Enter a verb: ");
    flushio();

    io::stdin()
        .read_line(&mut verb)
        .expect("Failed to read input");

    print!("Enter an adjective: ");
    flushio();

    io::stdin()
        .read_line(&mut adjective)
        .expect("Failed to read input");

    print!("Enter an adverb: ");
    flushio();

    io::stdin()
        .read_line(&mut adverb)
        .expect("Failed to read input");

    println!(
        "Do you {} your {} {} {}? That's hilarious!",
        verb.trim(),
        adjective.trim(),
        noun.trim(),
        adverb.trim()
    );
}

fn flushio() {
    io::stdout().flush().expect("");
}
