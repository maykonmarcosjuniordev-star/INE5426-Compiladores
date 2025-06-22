use std::error::Error;
use std::rc::Rc;
use std::io::Write;

use crate::grammar::var_type::VarType;
use crate::scope_stack::ScopeStack;
use crate::grammar::semantic_node::SemanticNodeData;

#[derive(Debug, Clone, PartialEq)]
pub struct SemanticNode {
  pub scopes: Rc<ScopeStack>,
  pub children: SemanticNodeData
}

impl SemanticNode {
  fn semantic_analysis(&self) -> Result<(), Box<dyn Error>> {
    // TODO
    Ok(())
  }
}

pub struct SemanticTree {
  pub root: SemanticNode,
}

impl SemanticTree {
  pub fn semantic_analysis(&mut self) -> Result<(), Box<dyn Error>> {
    // Perform semantic analysis on the syntax tree
    // This is where we would check for variable declarations, types, etc.
    // For now, we will just print the structure of the semantic tree
    self.root.semantic_analysis()?;
    Ok(())
  }

  pub fn save(&self, path: &str) -> Result<(), Box<dyn Error>> {
    let mut file = std::fs::File::create(path)?;
    writeln!(file, "{:?}", self.root)?;
    Ok(())
  }

  pub fn generate_code(&self, path: &str) -> Result<(), Box<dyn Error>> {
    // TODO
    Ok(())
  }
}