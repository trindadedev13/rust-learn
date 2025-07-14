use std::collections::HashMap;
use crate::command::Command;
use crate::program::Program;

pub fn make_se_program() -> Program {
  let mut cmds: HashMap<String, Box<dyn Command>> = HashMap::new();
  cmds.insert(EchoCommand::new().get_name(), Box::new(EchoCommand::new()));
  Program {
    name: "system".to_string(),
    commands: cmds
  }
}

// Echo Command
// Usage: echo <Any>
#[derive(Clone)]
pub struct EchoCommand;

impl EchoCommand {
  pub fn new() -> EchoCommand {
    EchoCommand {}
  }
}

impl Command for EchoCommand {
  fn get_name(&self) -> String {
    "echo".to_string()
  }

  fn call(&self, name: &str, params: Vec<String>) -> String {
    for param in &params {
      print!("{} ", param);
    }
    "".to_string()
  }
}