use std::io;
use std::io::Write;


fn main() {
  let mut password_input = String::new();
  let password = "abc123".to_string();

  println!("What is the password?");
  io::stdout().flush().expect("");

  io::stdin()
    .read_line(&mut password_input)
    .expect("Failed to read input.");

  if password_input.trim() == password {
    println!("Welcome!");
  }else {
    println!("I don't know you.");
  }
}
