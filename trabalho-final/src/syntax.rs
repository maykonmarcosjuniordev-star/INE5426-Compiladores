use crate::token::Token;
use crate::grammar::token_type::TokenType;
use crate::grammar::non_terminals::NonTerminal;
use std::collections::HashMap;
use std::rc::Rc;
use std::io::Write;

#[derive(Clone, Copy)] 
pub enum Symbol {
  NonTerminal(NonTerminal),
  Terminal(TokenType),
}

impl std::fmt::Debug for Symbol {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Symbol::NonTerminal(nt) => write!(f, "{:?}", nt),
      Symbol::Terminal(tt) => write!(f, "{:?}", tt),
    }
  }
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

  fn parse(&mut self, tokens: &Vec<Token>, index: &mut usize) -> Result<(), Box<dyn std::error::Error>> {
    let current_token = &tokens[*index];
    match self.value {
      Symbol::Terminal(token) => {
        // If the token type matches the current token, move to the next token
        if token == current_token.token_type { 
          *index += 1;
          return Ok(());
        }
        else { return Err(format!("Erro sintático: esperava {:?}, mas encontrou {:?} na linha {}, coluna {}", token, current_token.token_type, current_token.line, current_token.column).into()); }
      }
      Symbol::NonTerminal(non_terminal) => {
        match self.parse_table.get(&(non_terminal, current_token.token_type)) {
          Some(&rule_index) => {
            match &self.rules[rule_index as usize].1 {
              Some(body) => {
                for symbol in body {
                  let mut child = Node::new(symbol.clone(), Rc::clone(&self.parse_table), Rc::clone(&self.rules));
                  child.parse(tokens, index)?;
                  self.children.push(child);
                }
                Ok(())
              },
              None => Ok(())
            }
          }
          None => return Err(format!("Erro sintático: não há regra para {:?} com o token {:?} na linha {}, coluna {}", non_terminal, current_token.token_type, current_token.line, current_token.column).into()),
        }
      }
    }
  }

  fn to_string(&self, count: &mut u32) -> String {
    let mut result = String::new();
    let node_name = format!("{:?}_{}", self.value, count);
    *count += 1;
    match self.value {
      Symbol::Terminal(token) => {
        result.push_str(&format!("  {} [label=\"{:?}\" color=\"blue\"]\n", node_name, token));
      },
      Symbol::NonTerminal(nt) => {
        result.push_str(&format!("  {} [label=\"{:?}\" color=\"green\"]\n", node_name, nt));
      },
    }
    match self.value {
      Symbol::Terminal(_token) => {},
      Symbol::NonTerminal(_nt) => {
        if self.children.is_empty() {
          result.push_str(&format!("  Empty_{} [label=\"ε\" color=\"gray\"]\n", count));
          result.push_str(&format!("  {} -> Empty_{}\n", node_name, count));
          *count += 1;
          return result;
        } else {
          for child in &self.children {
            let child_name = format!("{:?}_{}", child.value, count);
            result.push_str(&format!("  {} -> {}\n", node_name, child_name));
            result.push_str(&child.to_string(count));
          }
        }
      }    
    }
    result
  }
}

pub struct SyntaxTree {
  root: Node
}

impl SyntaxTree {
  pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
    // Load Grammar rules
    let rule_content = include_str!("../grammars/syntax.txt");
    let mut rules = vec![];
    for line in rule_content.lines() {
      let parts: Vec<&str> = line.split(",").collect();
      if parts.len() != 2 { continue; }
      let head = NonTerminal::from_str(parts[0])?;
      let body: Option<Vec<Symbol>> = match parts[1] {
        "''" => None,
        // The else case is when grammars/syntax.txt has an invalid rule, this problem
        // should be identified at compile time so that it's fixed in the grammar file instead of here.
        // Hopefully the else case will never be hit.  
        _ => Some(parts[1].split_whitespace().map(|s| {
          if let Ok(token) = TokenType::from_str(s) { Symbol::Terminal(token) }
          else if let Ok(nt) = NonTerminal::from_str(s) { Symbol::NonTerminal(nt) }
          else { panic!("Invalid grammar") }
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
    Ok(SyntaxTree { root })
  }

  pub fn parse(&mut self, tokens: &Vec<Token>) -> Result<(), Box<dyn std::error::Error>> {
    self.root.parse(tokens, &mut 0)?;
    Ok(())
  }

  pub fn save(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = std::fs::File::create(path)?;
    writeln!(file, "// Visualize a árvore colando este arquivo em https://dreampuf.github.io/GraphvizOnline/?engine=dot")?;
    writeln!(file, "digraph G {{")?;
    writeln!(file, "{}", self.root.to_string(&mut 0))?;
    writeln!(file, "}}")?;
    Ok(())
  }
}
