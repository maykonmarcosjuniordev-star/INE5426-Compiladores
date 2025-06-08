use crate::token_type::TokenType;

#[derive(Clone)]
pub enum ConstType {
  Int(i64),
  Float(f64),
  String(String),
  KeyWord(String),
}

impl ConstType {
  pub fn from_str(s: &str) -> ConstType {
    if let Ok(i) = s.parse::<i64>() { return ConstType::Int(i); }
    if let Ok(f) = s.parse::<f64>() { return ConstType::Float(f); }
    if s.starts_with('"') && s.ends_with('"') { return ConstType::String(s[1..s.len()-1].to_string()); }
    ConstType::KeyWord(s.to_string())
  }
}

impl std::fmt::Debug for ConstType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      ConstType::Int(i) => write!(f, "{}", i),
      ConstType::Float(fl) => write!(f, "{}", fl),
      ConstType::String(s) => write!(f, "\"{}\"", s),
      ConstType::KeyWord(s) => write!(f, "{}", s),
    }
  }
}

impl std::fmt::Display for ConstType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      ConstType::Int(i) => write!(f, "{}", i),
      ConstType::Float(fl) => write!(f, "{}", fl),
      ConstType::String(s) => write!(f, "{}", s),
      ConstType::KeyWord(s) => write!(f, "{}", s),
    }
  }
}

#[derive(Debug, Clone)]
pub struct Token {
  pub token_type: TokenType,
  pub value: Option<ConstType>,
  pub line: usize,
  pub column: usize,
}
