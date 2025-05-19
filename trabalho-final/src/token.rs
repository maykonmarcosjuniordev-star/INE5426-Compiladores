use std::error::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenType {
  ConstChar,
  ConstBool,
  ConstNull,
  Lparenthesis,
  Rparenthesis,
  Lbracket,
  Rbracket,
  Rbrace,
  Lbrace,
  Semicolon,
  Comma,
  VarType,
  KwIf,
  KwElif,
  KwElse,
  KwWhile,
  KwFor,
  KwBreak,
  KwContinue,
  KwReturn,
  KwDef,
  KwPrint,
  KwRead,
  OpAssign,
  OpEq,
  OpNe,
  OpGt,
  OpGe,
  OpLt,
  OpLe,
  OpAnd,
  OpOr,
  OpXor,
  OpNot,
  OpBitwiseAnd,
  OpBitwiseOr,
  OpBitwiseXor,
  OpBitwiseNot,
  OpLBitshift,
  OpRBitshift,
  OpPlus,
  OpMinus,
  OpMult,
  OpDivision,
  OpWholeDivision,
  OpModular,
  OpPow,
  Id,
  ConstInt,
  ConstFloat,
  ConstString,
  EOF
}

impl TokenType {
  pub fn from_str(s: &str) -> Result<TokenType, Box<dyn Error>> {
    match s {
      "comma" => Ok(TokenType::Comma),
      "const_bool" => Ok(TokenType::ConstBool),
      "const_char" => Ok(TokenType::ConstChar),
      "const_float" => Ok(TokenType::ConstFloat),
      "const_int" => Ok(TokenType::ConstInt),
      "const_null" => Ok(TokenType::ConstNull),
      "const_string" => Ok(TokenType::ConstString),
      "id" => Ok(TokenType::Id),
      "kw_bool" => Ok(TokenType::ConstBool),
      "kw_if" => Ok(TokenType::KwIf),
      "kw_elif" => Ok(TokenType::KwElif),
      "kw_else" => Ok(TokenType::KwElse),
      "kw_while" => Ok(TokenType::KwWhile),
      "kw_for" => Ok(TokenType::KwFor),
      "kw_break" => Ok(TokenType::KwBreak),
      "kw_continue" => Ok(TokenType::KwContinue),
      "kw_return" => Ok(TokenType::KwReturn),
      "kw_def" => Ok(TokenType::KwDef),
      "kw_print" => Ok(TokenType::KwPrint),
      "kw_read" => Ok(TokenType::KwRead),
      "lparenthesis" => Ok(TokenType::Lparenthesis),
      "rparenthesis" => Ok(TokenType::Rparenthesis),
      "lbracket" => Ok(TokenType::Lbracket),
      "rbracket" => Ok(TokenType::Rbracket),
      "rbrace" => Ok(TokenType::Lbrace),
      "lbrace" => Ok(TokenType::Rbrace),
      "semicolon" => Ok(TokenType::Semicolon),
      "var_type" => Ok(TokenType::VarType),
      "op_assign" => Ok(TokenType::OpAssign),
      "op_eq" => Ok(TokenType::OpEq),
      "op_ne" => Ok(TokenType::OpNe),
      "op_gt" => Ok(TokenType::OpGt),
      "op_ge" => Ok(TokenType::OpGe),
      "op_lt" => Ok(TokenType::OpLt),
      "op_le" => Ok(TokenType::OpLe),
      "op_and" => Ok(TokenType::OpAnd),
      "op_or" => Ok(TokenType::OpOr),
      "op_xor" => Ok(TokenType::OpXor),
      "op_not" => Ok(TokenType::OpNot),
      "op_bitwise_and" => Ok(TokenType::OpBitwiseAnd),
      "op_bitwise_or" => Ok(TokenType::OpBitwiseOr),
      "op_bitwise_xor" => Ok(TokenType::OpBitwiseXor),
      "op_bitwise_not" => Ok(TokenType::OpBitwiseNot),
      "op_l_bitshift" => Ok(TokenType::OpLBitshift),
      "op_r_bitshift" => Ok(TokenType::OpRBitshift),
      "op_plus" => Ok(TokenType::OpPlus),
      "op_minus" => Ok(TokenType::OpMinus),
      "op_mult" => Ok(TokenType::OpMult),
      "op_division" => Ok(TokenType::OpDivision),
      "op_whole_division" => Ok(TokenType::OpWholeDivision),
      "op_modular" => Ok(TokenType::OpModular),
      "op_pow" => Ok(TokenType::OpPow),
      "eof" => Ok(TokenType::EOF),
      _ => Err(format!("Invalid TokenType: {}", s).into()),
    }
  }

  pub fn has_value(&self) -> bool {
    match self {
      TokenType::ConstChar | TokenType::ConstBool | TokenType::ConstInt | TokenType::ConstFloat | TokenType::ConstString | TokenType::Id | TokenType::VarType => true,
      _ => false,
    }
  }
}

#[derive(Clone)]
pub enum ConstType {
  Char(char),
  Bool(bool),
  Int(i64),
  Float(f64),
  String(String),
  KeyWord(String),
}

impl ConstType {
  pub fn from_str(s: &str) -> ConstType {
    if s.starts_with('\'') && s.ends_with('\'') && s.len() == 3 { return ConstType::Char(s.chars().nth(1).unwrap()); }
    if s == "true" { return ConstType::Bool(true); }
    if s == "false" { return ConstType::Bool(false); }
    if let Ok(i) = s.parse::<i64>() { return ConstType::Int(i); }
    if let Ok(f) = s.parse::<f64>() { return ConstType::Float(f); }
    if s.starts_with('"') && s.ends_with('"') { return ConstType::String(s[1..s.len()-1].to_string()); }
    ConstType::KeyWord(s.to_string())
  }
}

impl std::fmt::Debug for ConstType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      ConstType::Char(c) => write!(f, "'{}'", c),
      ConstType::Bool(b) => write!(f, "{}", b),
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
      ConstType::Char(c) => write!(f, "{}", c),
      ConstType::Bool(b) => write!(f, "{}", b),
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
