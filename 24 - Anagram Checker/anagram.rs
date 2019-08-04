use std::io;
use std::io::Write;

fn main() {
    println!("Enter two strings and I'll tell you if they are anagrams:");

    let first_string = get_input("first".to_string());
    let second_string = get_input("second".to_string());

    let is_anagram = is_anagram(first_string.to_string(), second_string.to_string());

    println!(
        "\"{}\" and \"{}\" are{} anagrams.",
        first_string,
        second_string,
        output(is_anagram)
    );
}

fn is_anagram(first_string: String, second_string: String) -> bool {
    let mut first_string_chars: Vec<char> = first_string.chars().collect();
    let mut second_string_chars: Vec<char> = second_string.chars().collect();

    first_string.len() == second_string.len()
        && first_string_chars.sort() == second_string_chars.sort()
}

fn output(is_anagram: bool) -> String {
    if is_anagram {
        "".to_string()
    } else {
        " not".to_string()
    }
}

fn get_input(output: String) -> String {
    print!("Enter the {} string: ", output);

    let mut input = String::new();
    io::stdout().flush().expect("");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_lowercase()
}
