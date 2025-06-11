use crate::grammar::token_type::TokenType;

#[derive(Clone, Debug)]
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

#[derive(Clone)]
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