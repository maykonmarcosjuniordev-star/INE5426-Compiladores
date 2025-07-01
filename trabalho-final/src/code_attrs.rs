#[allow(dead_code)]
pub struct CodeAttrs {
  pub register_counter: u32,
  pub label_counter: u32,
  pub code: String,
  pub memory_params: Vec<usize>,
  tmp_return: String,
  break_label: String,
}

impl CodeAttrs {
  pub fn new() -> Self {
    CodeAttrs {
      register_counter: 0,
      label_counter: 0,
      break_label: String::new(),
      tmp_return: String::new(),
      code: String::new(),
      memory_params: vec![],
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

  pub fn get_prev_return(&self) -> &String {
    &self.tmp_return
  }

  pub fn set_prev_return(&mut self, tmp: String) {
    self.tmp_return = tmp;
  }

  pub fn get_scope_label(&self) -> &String {
    &self.break_label
  }

  pub fn set_scope_end(&mut self, label: String) {
    self.break_label = label;
  }
}