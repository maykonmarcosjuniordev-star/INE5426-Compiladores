#[derive(Clone, Debug, PartialEq)]
pub enum ConstType {
  Int(i64),
  Float(f64),
  String(String),
}

impl ConstType {
  pub fn from_str(s: &str) -> ConstType {
    if let Ok(i) = s.parse::<i64>() { return ConstType::Int(i); }
    if let Ok(f) = s.parse::<f64>() { return ConstType::Float(f); }
    ConstType::String(s.to_string())
  }

  pub fn get_type(&self) -> VarType {
    match self {
      ConstType::Int(_) => VarType::Int,
      ConstType::Float(_) => VarType::Float,
      ConstType::String(_) => VarType::String,
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum VarType {
  Int,
  Float,
  String
}