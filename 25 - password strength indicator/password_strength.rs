use std::io;
use std::io::Write;

fn main() {
    let mut password_input = String::new();
    println!("Enter the password");
    io::stdout().flush().expect("");

    io::stdin()
        .read_line(&mut password_input)
        .expect("Failed to read input");
    let password = password_input.trim().to_string();

    let password_strength = password_validator(password);
    let strength_output = strength_output(password_strength);

    println!("The password '{}' is a {} password", password_input.trim(), strength_output);
}

fn password_validator(password: String) -> usize {
    let mut password_chars = password.chars();

    if password.len() < 8 {
        if password_chars.all(|char| char.is_numeric()) {
            return 1
        }
    } else {
        if password_chars.any(|char| char.is_alphabetic()) &&
            password_chars.any(|char| char.is_numeric()) {
            if password_chars.all(|char| ! char.is_alphanumeric()) {
                return 3
            } else {
                return 4
            }
        }
    }
    2
}

fn strength_output(strength: usize) -> String {
    match strength {
        1 => "very weak".to_string(),
        2 => "weak".to_string(),
        3 => "strong".to_string(),
        4 => "very strong".to_string(),
        _ => "".to_string(),
    }
}

#[test] fn test(){
    assert_eq!(2, 2);
}
