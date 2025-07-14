pub trait Command: CommandClone {
  fn get_name(&self) -> String;
  fn call(&self, name: &str, params: Vec<String>) -> String;
}

pub trait CommandClone {
  fn clone_box(&self) -> Box<dyn Command>;
}

impl<T> CommandClone for T
where
    T: 'static + Command + Clone,
{
  fn clone_box(&self) -> Box<dyn Command> {
    Box::new(self.clone())
  }
}

impl Clone for Box<dyn Command> {
  fn clone(&self) -> Box<dyn Command> {
    self.clone_box()
  }
}