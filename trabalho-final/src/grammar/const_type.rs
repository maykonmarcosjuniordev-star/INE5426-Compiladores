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

  pub fn get_keyword_type(&self) -> VarType {
    let ConstType::String(s) = self else { panic!("Expected ConstType::String"); };
    match s.as_str() {
      "int" => VarType::Int,
      "float" => VarType::Float,
      "string" => VarType::String,
      _ => panic!("Unknown type keyword: {}", s),
    }
  }
  
  pub fn to_string(&self) -> String {
    match self {
      ConstType::Int(i) => i.to_string(),
      ConstType::Float(f) => f.to_string(),
      ConstType::String(s) => s.replace("\"", "\\\"")
    }
  }
}

impl std::fmt::Display for ConstType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      ConstType::Int(i) => write!(f, "{}", i),
      ConstType::Float(fl) => write!(f, "{}", fl),
      ConstType::String(s) => write!(f, "{}", s),
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum VarType {
  Int,
  Float,
  String
}
