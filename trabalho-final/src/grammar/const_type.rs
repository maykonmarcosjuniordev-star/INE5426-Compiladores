#[derive(Clone, Debug, PartialEq)]
pub enum ConstType {
  Int(i64),
  Float(f64),
  String(String),
  VarType(VarType),
}

impl ConstType {
  pub fn from_str(s: &str) -> ConstType {
    if let Ok(i) = s.parse::<i64>() { return ConstType::Int(i); }
    if let Ok(f) = s.parse::<f64>() { return ConstType::Float(f); }
    if s == "int" { return ConstType::VarType(VarType::Int); }
    if s == "float" { return ConstType::VarType(VarType::Float); }
    if s == "string" { return ConstType::VarType(VarType::String); }
    ConstType::String(s.to_string())
  }

  pub fn get_type(&self) -> VarType {
    match self {
      ConstType::Int(_) => VarType::Int,
      ConstType::Float(_) => VarType::Float,
      ConstType::String(_) => VarType::String,
      ConstType::VarType(v) => v.clone(),
    }
  }

  pub fn get_keyword_type(&self) -> VarType {
    match self {
      ConstType::VarType(v) => v.clone(),
      _ => panic!("Expected VarType"),
    }
  }
  
  pub fn to_string(&self) -> String {
    match self {
      ConstType::Int(i) => i.to_string(),
      ConstType::Float(f) => f.to_string(),
      ConstType::String(s) => s.replace("\"", "\\\""),
      ConstType::VarType(v) => format!("{}", v),
    }
  }
}

impl std::fmt::Display for ConstType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      ConstType::Int(i) => write!(f, "{}", i),
      ConstType::Float(fl) => write!(f, "{}", fl),
      ConstType::String(s) => write!(f, "{}", s),
      ConstType::VarType(v) => write!(f, "{}", v),
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum VarType {
  Int,
  Float,
  String
}

impl std::fmt::Display for VarType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      VarType::Int => write!(f, "int"),
      VarType::Float => write!(f, "float"),
      VarType::String => write!(f, "string"),
    }
  }
}
