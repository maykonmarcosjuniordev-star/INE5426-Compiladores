use crate::token::TokenType;
use std::collections::HashMap;

#[derive(Debug)] 
pub enum Symbol {
  NonTerminal(NonTerminal),
  Terminal(TokenType),
}

pub type ParseTable = HashMap<(NonTerminal, TokenType), u32>;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum NonTerminal {
  E,
  E_,
  T,
  T_,
  F,
}

impl NonTerminal {
  pub fn from_char(s: &str) -> Result<Self, Box<dyn std::error::Error>> {
    match s {
      "E" => Ok(NonTerminal::E),
      "E_" => Ok(NonTerminal::E_),
      "T" => Ok(NonTerminal::T),
      "T_" => Ok(NonTerminal::T_),
      "F" => Ok(NonTerminal::F),
      _ => Err("Invalid non-terminal".into()),
    }
  }
}
