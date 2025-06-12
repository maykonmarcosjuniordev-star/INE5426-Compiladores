use crate::token::Token;
use crate::grammar::token_type::TokenType;
use crate::grammar::non_terminals::NonTerminal;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone, Copy)] 
pub enum Symbol {
  NonTerminal(NonTerminal),
  Terminal(TokenType),
}

pub type ParseTable = HashMap<(NonTerminal, TokenType), u32>;


struct Node {
  value: Symbol,
  children: Vec<Box<Node>>,
  parse_table: Rc<ParseTable>,
  rules: Rc<Vec<(NonTerminal, Option<Vec<Symbol>>)>>,
}

impl Node {
  fn new(
    value: Symbol,
    parse_table: Rc<HashMap<(NonTerminal, TokenType), u32>>,
    rules: Rc<Vec<(NonTerminal, Option<Vec<Symbol>>)>>
  ) -> Box<Self> {
    Box::new(Node {
      value,
      children: vec![],
      parse_table,
      rules
    })
  }

  fn parse(&mut self, tokens: &Vec<Token>, index: usize) -> usize {
    let current_token = &tokens[index];
    match self.value {
      Symbol::Terminal(token) => {
        // If the token type matches the current token, move to the next token
        if token == current_token.token_type { return index + 1; }
        else { panic!("Syntax error: expected {:?}, found {:?} at line {} column {}", token, current_token.token_type, current_token.line, current_token.column); }
      }
      Symbol::NonTerminal(non_terminal) => {
        match self.parse_table.get(&(non_terminal, current_token.token_type)) {
          Some(&rule_index) => {
            match &self.rules[rule_index as usize].1 {
              Some(body) => {
                let mut new_index = index;
                for symbol in body {
                  let mut child = Node::new(symbol.clone(), Rc::clone(&self.parse_table), Rc::clone(&self.rules));
                  new_index = child.parse(tokens, new_index);
                  self.children.push(child);
                }
                new_index
              },
              None => index
            }
          }
          None => panic!("Syntax error: no rule for {:?} with token {:?}", non_terminal, current_token.token_type),
        }
      }
    }
  }

  fn print(&self) -> String {
    match self.value {
      Symbol::Terminal(token) => format!("{:?}", token),
      Symbol::NonTerminal(_) => {
        let mut result = format!("{:?}, Children(", self.value);
        for child in &self.children {
          result.push_str(&format!("{},", child.print()));
        }
        // Remove the last comma
        result = result.trim_end_matches(',').to_string();
        result.push(')');
        result
      }
    }
  }
}

pub struct SyntaxTree {
  root: Node,
  _parse_table: Rc<HashMap<(NonTerminal, TokenType), u32>>,
  _rules: Rc<Vec<(NonTerminal, Option<Vec<Symbol>>)>>,
}

impl SyntaxTree {
  pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
    // Load Grammar rules
    let rule_file = std::fs::read_to_string("grammars/syntax.txt")?;
    let mut rules = vec![];
    for line in rule_file.lines() {
      let parts: Vec<&str> = line.split(",").collect();
      if parts.len() != 2 { continue; }
      let head = NonTerminal::from_str(parts[0])?;
      let body: Option<Vec<Symbol>> = match parts[1] {
        "''" => None,
        _ => Some(parts[1].split_whitespace().map(|s| {
          if let Ok(token) = TokenType::from_str(s) { Symbol::Terminal(token) }
          else { Symbol::NonTerminal(NonTerminal::from_str(s).unwrap()) }
        }).collect()),
      };
      rules.push((head, body));
    }
    // Load LL1 Parse Table
    let parse_table_file = std::fs::read_to_string("grammars/parse-table.txt")?;
    let mut parse_table = HashMap::new();
    for line in parse_table_file.lines() {
      let parts: Vec<&str> = line.split(",").collect();
      if parts.len() != 3 { continue; }
      let head = NonTerminal::from_str(parts[0])?;
      let token = TokenType::from_str(parts[1])?;
      let rule_index = parts[2].parse::<u32>()?;
      parse_table.insert((head, token), rule_index);
    }
    // Create the root node
    let rules = Rc::new(rules);
    let parse_table = Rc::new(parse_table);
    let root = Node { 
      value: Symbol::NonTerminal(NonTerminal::Program),
      children: vec![],
      parse_table: Rc::clone(&parse_table),
      rules: Rc::clone(&rules)
    };
    Ok(SyntaxTree { root, _rules: rules, _parse_table: parse_table })
  }

  pub fn parse(&mut self, tokens: &Vec<Token>) {
    self.root.parse(tokens, 0);
  }

  pub fn print(&self) {
    println!("Parse tree: {:?}", self.root.print());
  }
}
