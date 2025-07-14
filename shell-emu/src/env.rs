use std::collections::HashMap;
use crate::program::Program;

pub struct Environment {
  programs: HashMap<String, Program>
}

impl Environment {
  pub fn new()-> Environment {
    Environment { programs: HashMap::new() }
  }

  pub fn add_program(&mut self, program: Program) {
    self.programs.insert(program.name.clone(), program);
  }

  pub fn run_command(&self, program_name: &str, command_name: &str, params: Vec<String>) {
    let program = self.programs[program_name].clone();
    if let Some(command) = program.commands.get(command_name) {
      command.call(command_name, params);
    } else {
      println!("Unknown command: {}", command_name)
    }
  }
}