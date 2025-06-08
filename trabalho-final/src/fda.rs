use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::error::Error;

use crate::token_type::TokenType;

type State = u32;
type Symbol = char;

fn byte_vec_into_u32(vec: &Vec<u8>) -> u32 {
  let mut result = 0;
  for byte in vec.iter() {
    result *= 256;
    result += *byte as u32;
  }
  result
}

pub struct FDA {
  pub initial_state: State,
  pub transitions: HashMap<(State, Symbol), State>,
  pub token_table: HashMap<State, TokenType>,
}

impl FDA {
  pub fn new(initial_state: State, transitions: HashMap<(State, Symbol), State>, token_table: HashMap<State, TokenType>) -> FDA {

    FDA { initial_state, transitions, token_table }
  }

  pub fn from_file() -> Result<FDA, Box<dyn Error>> {
    // /machines/lexer.automata precisa existir durante a compilação do projeto
    // O mesmo vale para /machines/lexer_table.automata
    let raw_bytes = include_bytes!("../machines/lexer.automata");
    let mut transitions: HashMap<(State, Symbol), State> = HashMap::new();

    // The first byte is the number of bytes per state
    // Hopefully no automaton will ever need more than 256 bytes to encode its states
    let state_size = raw_bytes[0] as usize;
    // Next groups of bytes are the transitions, in the format
    // (state, symbol, next_state). Each symbol is a single byte
    let mut i = 1;
    while i < raw_bytes.len() {
      match raw_bytes.get(i..i+2*state_size+1) {
        Some(transition) => {
          let state = byte_vec_into_u32(&transition[..state_size].try_into().unwrap());
          let symbol = transition[state_size] as char;
          let next_state = byte_vec_into_u32(&transition[state_size+1..2*state_size+1].try_into().unwrap());
          let transition = next_state;
          transitions.insert((state, symbol), transition);
          i += 2*state_size + 1;
        },
        None => break,
      }
    }
    
    // Read the token table
    let token_table_file = File::open("machines/lexer_table.automata")?;
    let reader = BufReader::new(token_table_file);
    let mut token_table = HashMap::new();
    for line in reader.lines() {
      let line = line?;
      let parts: Vec<&str> = line.split(':').collect();
      if parts.len() != 2 { return Err("Invalid token table format".into()); }
      let state = parts[0].parse::<u32>()?;
      let token = TokenType::from_str(parts[1])?;
      token_table.insert(state, token);
    }

    let fda = FDA::new(0, transitions, token_table);
    Ok(fda)
  }

  pub fn transtion(&self, state: State, symbol: Symbol) -> Option<&State> {
    if self.transitions.contains_key(&(state, symbol)) { self.transitions.get(&(state, symbol)) }
    // Group transitions: If the specific character doesn't have a transition, check if there's a transition for a group in which the character belongs
    // Yeah for now there are no groups and I'm not sure if there'll ever be any.
    // If the all groups above failed, check for the wildcard symbol. It skips any check and just runs the transition for whatever symbol it has read
    else if self.transitions.contains_key(&(state, '\x00')) { self.transitions.get(&(state, '\x00')) }
    else { None }
  }
}