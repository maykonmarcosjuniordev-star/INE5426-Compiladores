use std::error::Error;
use std::rc::Rc;
use std::io::Write;

use crate::code_attrs::CodeAttrs;
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

  fn generate_code(&self, inh: &mut CodeAttrs) {
    match &self.children {
      SemanticNodeData::Program { funclist, statement } => {
        if let Some(funclist) = funclist {
          funclist.generate_code(inh);
        } else if let Some(statement) = statement {
          statement.generate_code(inh);
        }
      },
      _ => panic!()
    }
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
    let mut file = std::fs::File::create(path)?;
    let mut code_attrs = CodeAttrs {
      register_counter: 0,
      label_counter: 0,
      code: String::new(),
    };
    self.root.generate_code(&mut code_attrs);
    writeln!(file, "{}", code_attrs.code)?;
    Ok(())
  }
}