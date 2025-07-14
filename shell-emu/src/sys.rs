use std::process::Command;
use std::io::{self, Write};

pub fn clear() {
  let output = Command::new("clear")
    .output()
    .expect("Failed to clear screen");

  io::stdout()
    .write_all(&output.stdout)
    .expect("Failed to write to stdout");
}