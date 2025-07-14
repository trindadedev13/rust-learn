mod command;
mod default;
mod env;
mod program;
mod sys;

use std::io::{self, Write};
use crate::default::make_se_program;

fn welcome() {
  println!("Welcome to Shell-Emulator!");
  println!("Shell Emulator is a Terminal Emulator with its own environment");
}

fn main() {
  sys::clear();
  welcome();

  let mut env = env::Environment::new();
  env.add_program(make_se_program());

  loop {
    print!("=> ");
    io::stdout()
      .flush()
      .unwrap();

    let mut raw_input = String::new();
    io::stdin()
      .read_line(&mut raw_input)
      .expect("Failed to get input");

    let input: Vec<&str> = raw_input.trim().split_whitespace().collect();
    if input.len() > 0 {
      if !input[0].is_empty() {
        if input[0] == "exit" {
          break
        }
        env.run_command(
          "system",
          input[0],
          input[1..].iter().map(|s| s.to_string()).collect()
        );
      }
    }
  }
}