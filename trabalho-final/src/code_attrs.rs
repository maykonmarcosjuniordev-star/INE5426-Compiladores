#[allow(dead_code)]
pub struct CodeAttrs {
  register_counter: u32,
  label_counter: u32,
  break_label: String,
  pub code: String,
}

impl CodeAttrs {
  pub fn new() -> Self {
    CodeAttrs {
      register_counter: 0,
      label_counter: 0,
      break_label: String::new(),
      code: String::new(),
    }
  }

  pub fn create_temp(&mut self) -> String {
    self.register_counter += 1;
    let temp = format!("t{}", self.register_counter);
    temp
  }

  pub fn create_label(&mut self) -> String {
    self.label_counter += 1;
    let label = format!("L{}", self.label_counter);
    label
  }

  pub fn get_scope_label(&self) -> &String {
    &self.break_label
  }

  pub fn set_scope_end(&mut self, label: String) {
    self.break_label = label;
  }
}