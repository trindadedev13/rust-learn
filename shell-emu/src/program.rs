use std::collections::HashMap;
use crate::command::Command;

#[derive(Clone)]
pub struct Program {
  pub name: String,
  pub commands: HashMap<String, Box<dyn Command>>
}

impl Program {
  pub fn new(name: String) -> Program {
    Program {
      name,
      commands: HashMap::new()
    }
  }
}