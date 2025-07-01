use crate::grammar::token_type::TokenType;
use crate::grammar::const_type::{VarType, ConstType};

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
      Some(value) => write!(f, "token: {:?}, value: {:?}, line: {}, column: {}", self.token_type, value, self.line, self.column),
      None => write!(f, "token: {:?}, line: {}, column: {}", self.token_type, self.line, self.column),
    }
  }
}

impl std::fmt::Display for Token {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match &self.value {
      Some(value) => write!(f, "{}", value),
      None => write!(f, "{}", self.token_type),
    }
  }
}

impl Token {
  pub fn get_type(&self) -> VarType {
    match self.token_type {
      TokenType::ConstFloat => VarType::Float,
      TokenType::ConstInt => VarType::Int,
      TokenType::ConstString => VarType::String,
      TokenType::VarType => {
        if let Some(s) = &self.value { s.get_keyword_type() }
        else { panic!("Expected VarType value for VarType token"); }
      }
      _ => panic!(),
    }
  }
}