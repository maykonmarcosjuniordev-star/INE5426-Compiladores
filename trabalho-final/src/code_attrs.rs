#[allow(dead_code)]
pub struct CodeAttrs {
  pub register_counter: u32,
  pub label_counter: u32,
  pub last_temp: String,
  pub code: String,
  pub memory_params: Vec<usize>
}

impl CodeAttrs {
  pub fn new() -> Self {
    CodeAttrs {
      register_counter: 0,
      label_counter: 0,
      last_temp: String::new(),
      code: String::new(),
      memory_params: vec![],
    }
  }

  pub fn reset(&mut self) {
    self.register_counter = 0;
    self.label_counter = 0;
    self.last_temp.clear();
    self.code.clear();
  }

  pub fn create_temp(&mut self) -> String {
    self.register_counter += 1;
    let temp = format!("t{}", self.register_counter);
    self.last_temp = temp.clone();
    temp
  }    
}