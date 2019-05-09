use std::io;
use std::io::Write;
use std::str::FromStr;

fn main() {
  let mut age_input = String::new();

  println!("What is your age?");
  io::stdout().flush().expect("");

  io::stdin()
      .read_line(&mut age_input)
      .expect("Failed to read input.");

  let age = u8::from_str(age_input.trim()).expect("Please enter a valid age.");

  let output =
  if age >= 16 {
    "You are old enough to legally drive."
  }else {
    "You are not old enough to legally drive."
  };

  println!("{}", output);
}
