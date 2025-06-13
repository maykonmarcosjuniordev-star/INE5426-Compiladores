use crate::fda::FDA;
use crate::token::{ConstType, Token};
use crate::grammar::token_type::TokenType;
use std::collections::HashMap;
use std::error::Error;
use std::io::Write;

type State = u32;
pub struct Lexer {
  pub fda: FDA,
  pub token_list: TokenList,
  pub token_table: TokenTable,
  line_count: usize,
  column_count: usize,
  token_value: String,
  string: bool,
  current_state: State
}

pub type TokenList = Vec<Token>;
pub type TokenTable = HashMap<String, TokenEntry>;
pub type TokenEntry = Vec<(u32, u32)>;

impl Lexer {
  pub fn new() -> Lexer {
    let fda = FDA::from_file().expect("Lexer automata file not found");
    Lexer { 
      fda,
      token_list: vec![],
      token_table: HashMap::new(),
      line_count: 1,
      column_count: 0,
      token_value: String::new(),
      string: false,
      current_state: 0
    }
  }

  // TODO: this ffs
  // fn step(&mut self, char: char, use_state: Option<State>) {
  //   let next_state = match use_state {
  //     Some(state) => self.fda.transtion(state, char),
  //     None => self.fda.transtion(self.current_state, char)
  //   };

  //   match next_state {
  //     Some(next_state) => {},
  //     None => {}
  //   }
  // }

  pub fn parse(&mut self, input: &str) -> Result<(), Box<dyn Error>> {
    // TODO: Diferenciar ids de func_ids e armazenar em quais posições o token aparece.
    for char in input.chars() {
      // Keep track of current position in the input
      self.column_count += 1;
      if char == '"' { self.string = !self.string; }
      else if self.current_state == self.fda.initial_state && char.is_whitespace() { continue; }
      // Language only accepts uppercase letters inside of strings
      let character = if !self.string && char.is_alphabetic() { char.to_ascii_lowercase() } else { char };
      
      // Process the character
      let next_state = self.fda.transtion(self.current_state, character);
      match next_state {
        // If the transition is valid, just update the state and the token value
        Some(next_state) => {
          self.current_state = *next_state;
          self.token_value.push(character);
        },
        // If the transition is invalid, check if the current state is a final state
        // If it is, this means that a valid token was found
        // And the current symbol is the start of a possible new token
        // If it is not, this means that the token built until now is invalid and must be discarded
        None => {
          // If the current state is a final state, we have a valid token
          if self.fda.token_table.contains_key(&self.current_state) {
            let token_type = self.fda.token_table.get(&self.current_state).unwrap();
            let token = Token{
              token_type: *token_type,
              value: if token_type.has_value() {Some(ConstType::from_str(&self.token_value))} else { None },
              line: self.line_count,
              column: self.column_count-self.token_value.len(),
            };
            if token_type.is_id() { self.token_table.insert(self.token_value.clone(), vec![]); }
            self.token_list.push(token);
          } else {
            // If the current state is not a final state, we have an invalid token
            // Print an error message and discard the token
            return Err(format!("Error: Invalid token at line {}, column {}: '{}'", self.line_count, self.column_count, self.token_value).into());
          }

          self.token_value.clear();
          // Now that the previous token is stored
          // Check if the current character is a valid start of a token
          // If it is, execute the transition
          // If it is not, return the lexical error and reset the state
          if let Some(next_state) = self.fda.transitions.get(&(self.fda.initial_state, character)) {
            self.current_state = *next_state;
            if !character.is_whitespace() { self.token_value.push(character); }
          } else {
            // Since the compilation process halts at the first error, it doesn't need to reset the current state
            return Err(format!("Error: Invalid token at line {}, column {}: '{}'", self.line_count, self.column_count, self.token_value).into());
          }
        }
      }
      if char == '\n' { self.line_count += 1; self.column_count = 0; }
    }
    // If the last token is valid, add it to the list
    if self.fda.token_table.contains_key(&self.current_state) {
      let token_type = self.fda.token_table.get(&self.current_state).unwrap();
      let token = Token{
        token_type: *token_type,
        value: if token_type.has_value() {Some(ConstType::from_str(&self.token_value))} else { None },
        line: self.line_count,
        column: self.column_count-self.token_value.len(),
      };
      if token_type.is_id() { self.token_table.insert(self.token_value.clone(), vec![]); }
      self.token_list.push(token);
    }
    // If the last token is not valid, return an error
    else if !self.token_value.is_empty() {
      return Err(format!("Error: Invalid token at line {}, column {}: '{}'", self.line_count, self.column_count, self.token_value).into());
    }
    // Push EOF token to end of list for syntax analysis
    self.token_list.push(Token{
      token_type: TokenType::Eof,
      value: None,
      line: self.line_count,
      column: self.column_count,
    });

    Ok(())
  }

  pub fn save_token_list(&self, path: &str) -> Result<(), Box<dyn Error>> {
    let mut file = std::fs::File::create(path)?;
    for token in &self.token_list {
      writeln!(file, "{:?}", token)?;
    }
    Ok(())
  }

  pub fn save_token_table(&self, path: &str) -> Result<(), Box<dyn Error>> {
    let mut file = std::fs::File::create(path)?;
    for (key, value) in &self.token_table {
      writeln!(file, "{}: {:?}", key, value)?;
    }
    Ok(())
  }
}