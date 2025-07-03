pub struct ExpressionTree {
  pub root: ExpressionTreeNode
}

impl ExpressionTree {

  pub fn output(&self) -> String {
    let mut output = String::new();
    output.push_str("// Visualize a árvore colando este arquivo em https://dreampuf.github.io/GraphvizOnline/?engine=dot\n");
    output.push_str("digraph ExpressionTree {\n");
    let mut counter = 0;
    self.root.save(&mut output, &mut counter);
    output.push_str("}\n");
    output
  }
}

#[derive(Debug)]
pub enum ExpressionTreeNode {
  BinaryOperator {
    operator: Operator,
    left: Box<ExpressionTreeNode>,
    right: Box<ExpressionTreeNode>
  },
  UnaryOperator {
    operator: Operator,
    operand: Box<ExpressionTreeNode>
  },
  Operand {
    value: Operand
  }
}

impl ExpressionTreeNode {
  pub fn save(&self, output: &mut String, counter: &mut usize) {
    match self {
      ExpressionTreeNode::BinaryOperator { operator, left, right } => {
        let name = *counter;
        output.push_str(&format!("  {} [label=\"{}\"];\n", name, operator));
        output.push_str(&format!("  {} -> {};\n", name, *counter+1));
        *counter += 1;
        left.save(output, counter);
        output.push_str(&format!("  {} -> {};\n", name, *counter+1));
        *counter += 1;
        right.save(output, counter);
      },
      ExpressionTreeNode::UnaryOperator { operator, operand } => {
        let name = *counter;
        output.push_str(&format!("  {} [label=\"{}\"];\n", name, operator));
        output.push_str(&format!("  {} -> {};\n", name, *counter+1));
        *counter += 1;
        operand.save(output, counter);
      },
      ExpressionTreeNode::Operand { value } => {
        output.push_str(&format!("  {} [label=\"{}\"];\n", *counter, value));
        *counter += 1;
      }
    }
  }
}

#[derive(Debug)]
pub enum Operator {
  Eq,
  Ne,
  Gt,
  Ge,
  Lt,
  Le,
  Plus,
  Minus,
  Multiply,
  Division,
  Modular,
}

impl std::fmt::Display for Operator {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let symbol = match self {
      Operator::Eq => "==",
      Operator::Ne => "!=",
      Operator::Gt => ">",
      Operator::Ge => ">=",
      Operator::Lt => "<",
      Operator::Le => "<=",
      Operator::Plus => "+",
      Operator::Minus => "-",
      Operator::Multiply => "*",
      Operator::Division => "/",
      Operator::Modular => "%",
    };
    write!(f, "{}", symbol)
  }
}

#[derive(Debug)]
pub enum Operand {
  Integer(i64),
  Float(f64),
  String(String),
  Identifier(String),
}

impl std::fmt::Display for Operand {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let value = match self {
      Operand::Integer(i) => i.to_string(),
      Operand::Float(fl) => fl.to_string(),
      Operand::String(s) => s.replace("\"", "\\\""),
      Operand::Identifier(id) => if id.starts_with("@") {
        format!("Função {}", id)
      } else {
        format!("Variável {}", id)
      },
    };
    write!(f, "{}", value)
  }
}