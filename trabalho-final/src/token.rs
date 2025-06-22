use crate::grammar::token_type::TokenType;
use crate::grammar::const_type::ConstType;

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