// Esse arquivo foi gerado automaticamente pelo script scripts/consistency.py
use std::error::Error;
use crate::expression::Operator;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenType {
  Comma,
  ConstFloat,
  ConstInt,
  ConstNull,
  ConstString,
  Eof,
  FuncId,
  Id,
  KwBreak,
  KwDef,
  KwElse,
  KwFor,
  KwIf,
  KwNew,
  KwPrint,
  KwRead,
  KwReturn,
  Lbrace,
  Lbracket,
  Lparenthesis,
  OpAssign,
  OpDivision,
  OpEq,
  OpGe,
  OpGt,
  OpLe,
  OpLt,
  OpMinus,
  OpModular,
  OpMultiply,
  OpNe,
  OpPlus,
  Rbrace,
  Rbracket,
  Rparenthesis,
  Semicolon,
  VarType,
}

impl TokenType {
  pub fn from_str(s: &str) -> Result<TokenType, Box<dyn Error>> {
    match s {
      "comma" => Ok(TokenType::Comma),
      "const_float" => Ok(TokenType::ConstFloat),
      "const_int" => Ok(TokenType::ConstInt),
      "const_null" => Ok(TokenType::ConstNull),
      "const_string" => Ok(TokenType::ConstString),
      "eof" => Ok(TokenType::Eof),
      "func_id" => Ok(TokenType::FuncId),
      "id" => Ok(TokenType::Id),
      "kw_break" => Ok(TokenType::KwBreak),
      "kw_def" => Ok(TokenType::KwDef),
      "kw_else" => Ok(TokenType::KwElse),
      "kw_for" => Ok(TokenType::KwFor),
      "kw_if" => Ok(TokenType::KwIf),
      "kw_new" => Ok(TokenType::KwNew),
      "kw_print" => Ok(TokenType::KwPrint),
      "kw_read" => Ok(TokenType::KwRead),
      "kw_return" => Ok(TokenType::KwReturn),
      "lbrace" => Ok(TokenType::Lbrace),
      "lbracket" => Ok(TokenType::Lbracket),
      "lparenthesis" => Ok(TokenType::Lparenthesis),
      "op_assign" => Ok(TokenType::OpAssign),
      "op_division" => Ok(TokenType::OpDivision),
      "op_eq" => Ok(TokenType::OpEq),
      "op_ge" => Ok(TokenType::OpGe),
      "op_gt" => Ok(TokenType::OpGt),
      "op_le" => Ok(TokenType::OpLe),
      "op_lt" => Ok(TokenType::OpLt),
      "op_minus" => Ok(TokenType::OpMinus),
      "op_modular" => Ok(TokenType::OpModular),
      "op_multiply" => Ok(TokenType::OpMultiply),
      "op_ne" => Ok(TokenType::OpNe),
      "op_plus" => Ok(TokenType::OpPlus),
      "rbrace" => Ok(TokenType::Rbrace),
      "rbracket" => Ok(TokenType::Rbracket),
      "rparenthesis" => Ok(TokenType::Rparenthesis),
      "semicolon" => Ok(TokenType::Semicolon),
      "var_type" => Ok(TokenType::VarType),
      _ => Err(format!("Invalid TokenType: {}", s).into())
    }
  }

  pub fn has_value(&self) -> bool {
    match self {
      TokenType::ConstFloat | TokenType::ConstInt | TokenType::ConstString | TokenType::FuncId | TokenType::Id | TokenType::VarType => true,
      _ => false,
    }
  }

  pub fn is_id(&self) -> bool {
    match self {
      TokenType::Id | TokenType::FuncId => true,
      _ => false
    }
  }

  pub fn get_operator_type(&self) -> Operator {
    match self {
      TokenType::OpEq => Operator::Eq,
      TokenType::OpNe => Operator::Ne,
      TokenType::OpGt => Operator::Gt,
      TokenType::OpGe => Operator::Ge,
      TokenType::OpLt => Operator::Lt,
      TokenType::OpLe => Operator::Le,
      TokenType::OpPlus => Operator::Plus,
      TokenType::OpMinus => Operator::Minus,
      TokenType::OpMultiply => Operator::Multiply,
      TokenType::OpDivision => Operator::Division,
      TokenType::OpModular => Operator::Modular,

      _ => panic!()
    }
  }
}

impl std::fmt::Display for TokenType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      TokenType::Comma => write!(f, ","),
      TokenType::ConstFloat => write!(f, "const_float"),
      TokenType::ConstInt => write!(f, "const_int"),
      TokenType::ConstNull => write!(f, "null"),
      TokenType::ConstString => write!(f, "const_string"),
      TokenType::Eof => write!(f, "eof"),
      TokenType::FuncId => write!(f, "func_id"),
      TokenType::Id => write!(f, "id"),
      TokenType::KwBreak => write!(f, "break"),
      TokenType::KwDef => write!(f, "def"),
      TokenType::KwElse => write!(f, "else"),
      TokenType::KwFor => write!(f, "for"),
      TokenType::KwIf => write!(f, "if"),
      TokenType::KwNew => write!(f, "new"),
      TokenType::KwPrint => write!(f, "print"),
      TokenType::KwRead => write!(f, "read"),
      TokenType::KwReturn => write!(f, "return"),
      TokenType::Lbrace => write!(f, "{{"),
      TokenType::Lbracket => write!(f, "["),
      TokenType::Lparenthesis => write!(f, "("),
      TokenType::OpAssign => write!(f, "="),
      TokenType::OpDivision => write!(f, "/"),
      TokenType::OpEq => write!(f, "=="),
      TokenType::OpGe => write!(f, ">="),
      TokenType::OpGt => write!(f, ">"),
      TokenType::OpLe => write!(f, "<="),
      TokenType::OpLt => write!(f, "<"),
      TokenType::OpMinus => write!(f, "-"),
      TokenType::OpModular => write!(f, "%"),
      TokenType::OpMultiply => write!(f, "*"),
      TokenType::OpNe => write!(f, "!="),
      TokenType::OpPlus => write!(f, "+"),
      TokenType::Rbrace => write!(f, "}}"),
      TokenType::Rbracket => write!(f, "]"),
      TokenType::Rparenthesis => write!(f, ")"),
      TokenType::Semicolon => write!(f, ";"),
      TokenType::VarType => write!(f, "var_type"),
    }
  }
}
