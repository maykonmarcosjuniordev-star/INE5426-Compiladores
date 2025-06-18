use std::collections::HashMap;

use crate::grammar::token_type::TokenType;

pub type State = u32;
pub type Symbol = char;

pub fn byte_vec_into_u32(vec: &[u8]) -> u32 {
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
  pub fn transtion(&self, state: State, symbol: Symbol) -> Option<&State> {
    if self.transitions.contains_key(&(state, symbol)) { self.transitions.get(&(state, symbol)) }
    // Group transitions: If the specific character doesn't have a transition, check if there's a transition for a group in which the character belongs
    // Yeah for now there are no groups and I'm not sure if there'll ever be any.
    // If the all groups above failed, check for the wildcard symbol. It skips any check and just runs the transition for whatever symbol it has read
    else if self.transitions.contains_key(&(state, '\x00')) { self.transitions.get(&(state, '\x00')) }
    else { None }
  }
}

#[macro_export]
macro_rules! fda {
  ($transitions:expr, $token_table:expr) => {{
    let raw_bytes = include_bytes!($transitions);
    let mut transitions: HashMap<(State, crate::fda::Symbol), State> = HashMap::new();

    // The first byte is the number of bytes per state
    // Hopefully no automaton will ever need more than 256 bytes to encode its states
    let state_size = raw_bytes[0] as usize;
    // Next groups of bytes are the transitions, in the format
    // (state, symbol, next_state). Each symbol is a single byte
    let mut i = 1;
    while i < raw_bytes.len() {
      match raw_bytes.get(i..i+2*state_size+1) {
        Some(transition) => {
          let state = crate::fda::byte_vec_into_u32(&transition[..state_size]);
          let symbol = transition[state_size] as char;
          let next_state = crate::fda::byte_vec_into_u32(&transition[state_size+1..2*state_size+1]);
          let transition = next_state;
          transitions.insert((state, symbol), transition);
          i += 2*state_size + 1;
        },
        None => break,
      }
    }
    
    // Read the token table
    let token_table_content = include_str!($token_table);
    let mut token_table = HashMap::new();
    for line in token_table_content.lines() {
      let parts: Vec<&str> = line.split(':').collect();
      if parts.len() != 2 { panic!("Invalid token table format: {}", line); }
      let state = parts[0].parse::<u32>().expect("Invalid state in token table");
      let token = TokenType::from_str(parts[1]).expect("Invalid token type in token table");
      token_table.insert(state, token);
    }

    FDA {
      initial_state: 0, // The initial state is always 0
      transitions,
      token_table,
    }
  }}
}