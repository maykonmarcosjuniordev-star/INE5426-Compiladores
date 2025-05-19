use crate::token::TokenType;
use crate::cfg::{NonTerminal, Symbol, ParseTable};
use std::collections::HashMap;

#[derive(Debug)]
struct Node {
  value: Symbol,
  children: Vec<Box<Node>>,
  parse_table: ParseTable
}

impl Node {
  fn new(value: Symbol, parse_table: HashMap<(NonTerminal, TokenType), u32>) -> Box<Self> {
    Box::new(Node {
      value,
      children: vec![],
      parse_table,
    })
  }

  // fn parse(&mut self, tokens: Vec<Token>, index: usize) -> (usize, u32) {
  //   let stack_top = self.token.unwrap().token_type;
  //   let current_token = &tokens[index];
    
  //   // If the node represents the terminal symbol, this is a leaf node
  //   if stack_top == current_token.token_type { return (index + 1, 1); }
  //   // Get children of the node from the parsing table
  //   let children = PARSE_TABLE[stack_top as usize][current_token.token_type as usize];
  //   let mut index = index;
  //   for child in children {
  //     let child_node = Node::new(Some(Token {
  //       token_type: *child,
  //       lexeme: String::new(), // Placeholder for lexeme
  //       line: 0, // Placeholder for line number
  //     }));
  //     let (next_index, child_count) = child_node.parse(tokens.clone(), index);
  //     index = next_index;
  //     self.children.push(child_node);
  //   }
  //   (index, children.len() as u32)
  // }
}

#[derive(Debug)]
pub struct SyntaxTree {
  root: Node,
  parse_table: HashMap<(NonTerminal, TokenType), u32>,
  rules: Vec<(NonTerminal, Option<Vec<Symbol>>)>,
}

impl SyntaxTree {
  pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
    // Load Grammar rules
    let rule_file = std::fs::read_to_string("grammars/test-syntax.txt")?;
    let mut rules = vec![];
    for line in rule_file.lines() {
      let parts: Vec<&str> = line.split(",").collect();
      if parts.len() != 2 { continue; }
      let head = NonTerminal::from_char(parts[0])?;
      let body: Option<Vec<Symbol>> = match parts[1] {
        " " => None,
        _ => Some(parts[1].split_whitespace().map(|s| {
          if let Ok(token) = TokenType::from_str(s) { Symbol::Terminal(token) }
          else { Symbol::NonTerminal(NonTerminal::from_char(s).unwrap()) }
        }).collect()),
      };
      rules.push((head, body));
    }
    // Load LL1 Parse Table
    let parse_table_file = std::fs::read_to_string("grammars/test-parse-table.txt")?;
    let mut parse_table = HashMap::new();
    for line in parse_table_file.lines() {
      let parts: Vec<&str> = line.split(",").collect();
      if parts.len() != 3 { continue; }
      let head = NonTerminal::from_char(parts[0])?;
      let token = TokenType::from_str(parts[1])?;
      let rule_index = parts[2].parse::<u32>()?;
      parse_table.insert((head, token), rule_index);
    }
    // Create the root node
    let root = Node { 
      value: Symbol::NonTerminal(NonTerminal::E),
      children: vec![],
      parse_table: parse_table.clone()
    };
    Ok(SyntaxTree { root, rules, parse_table })
  }

  // pub fn parse(&mut self, tokens: Vec<Token>) {
  //   self.root.parse(tokens, 0);
  // }

}

