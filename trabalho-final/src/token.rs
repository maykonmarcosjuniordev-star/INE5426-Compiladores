use crate::grammar::token_type::TokenType;
#[allow(dead_code)]

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
}

#[derive(Clone, PartialEq)]
pub struct Token {
  pub token_type: TokenType,
  pub value: Option<ConstType>,
  pub line: usize,
  pub column: usize,
}

impl std::fmt::Debug for Token {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match &self.value {
      Some(value) => write!(f, "Token {{ type: {:?}, value: {:?} }}", self.token_type, value),
      None => write!(f, "Token {{ type: {:?} }}", self.token_type)
    }
  }
}