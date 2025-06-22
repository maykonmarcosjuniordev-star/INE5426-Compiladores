use std::error::Error;

use crate::scope_stack::ScopeStack;

pub struct SemanticNode {
  scopes: ScopeStack,
  node: SemanticNodeData
}

pub enum SemanticNodeData {
  Program {
    FUNCLIST: Option<Box<SemanticNodeData>>,
    STATEMENT: Option<Box<SemanticNodeData>>,
  },
  FUNCLIST {
    FUNDEF_FUNCLIST: Option<(Box<SemanticNodeData>, Box<SemanticNodeData>)>,
  },

}

impl SemanticNode {
  fn semantic_analysis(&self) -> Result<(), Box<dyn Error>> {
    // TODO
    Ok(())
  }
}


pub struct SemanticTree {
  root: SemanticNode,
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
    // TODO

    Ok(())
  }

  pub fn generate_code(&self, path: &str) -> Result<(), Box<dyn Error>> {
    // TODO
    Ok(())
  }
}