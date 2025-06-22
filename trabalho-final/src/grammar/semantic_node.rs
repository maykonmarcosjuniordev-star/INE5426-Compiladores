use crate::grammar::token_type::TokenType;
use crate::token::{ConstType, Token};
use crate::semantic::SemanticNode;

#[derive(Debug, Clone, PartialEq)]
pub enum SemanticNodeData {
  Allocexpression {
    var_type: Box<SemanticNode>,
    dimensions: Box<SemanticNode>,
  },
  Atribstat {
    lvalue: Box<SemanticNode>,
    value: Box<SemanticNode>,
  },
  Atribstatevalue {
    expression: Option<Box<SemanticNode>>,
    allocexpression: Option<Box<SemanticNode>>,
    funccall: Option<Box<SemanticNode>>,
  },
  Constant {
    value: ConstType
  },
  ConstIndex {
    index: Vec<SemanticNode>,
  },
  Elsestat {
    statement: Box<SemanticNode>,
  },
  Expression {
    numexpression: Box<SemanticNode>,
    op_expression: Option<Box<SemanticNode>>,
    numexpression2: Option<Box<SemanticNode>>,
  },
  Factor {
    expression: Option<Box<SemanticNode>>,
    lvalue: Option<Box<SemanticNode>>,
    constant: Option<Box<SemanticNode>>,
  },
  Forstat {
    init: Box<SemanticNode>,
    condition: Box<SemanticNode>,
    increment: Box<SemanticNode>,
    body: Box<SemanticNode>,
  },
  Funccall {
    id: Box<SemanticNode>,
    paramlistcall: Option<Box<SemanticNode>>,
  },
  Funcdef {
    func_id: Box<SemanticNode>,
    paramlist: Option<Box<SemanticNode>>,
    statelist: Box<SemanticNode>,
  },
  Funclist {
    funclist: Vec<SemanticNode>,
  },
  Ifstat {
    condition: Box<SemanticNode>,
    then_branch: Box<SemanticNode>,
    else_branch: Option<Box<SemanticNode>>,
  },
  Lvalue {
    id: Box<SemanticNode>,
    var_index: Option<Box<SemanticNode>>,
  },
  Numexpression {
    term: Box<SemanticNode>,
    op_numexpression: Option<Box<SemanticNode>>,
    term2: Option<Box<SemanticNode>>,
  },
  OpExpression {
    op: TokenType
  },
  OpNumexpression {
    op: TokenType
  },
  OpTerm {
    op: TokenType
  },
  Paramlist {
    paramlist: Vec<SemanticNode>,
  },
  Paramlistcall {
    paramlist: Vec<SemanticNode>,
  },
  Printstat {
    expression: Box<SemanticNode>,
  },
  Program {
    funclist: Option<Box<SemanticNode>>,
    statement: Option<Box<SemanticNode>>,
  },
  Readstat {
    lvalue: Box<SemanticNode>,
  },
  Returnstat,
  Statelist {
    statelist: Vec<SemanticNode>,
  },
  Statement {
    vardecl: Option<Box<SemanticNode>>,
    atribstat: Option<Box<SemanticNode>>,
    ifstat: Option<Box<SemanticNode>>,
    forstat: Option<Box<SemanticNode>>,
    statelist: Option<Box<SemanticNode>>,
    commandstat: Option<Box<SemanticNode>>,
  },
  Term {
    factor: Box<SemanticNode>,
    op_term: Option<Box<SemanticNode>>,
    factor2: Option<Box<SemanticNode>>,
  },
  Unaryexpression {
    op: Option<Box<SemanticNode>>,
    factor: Box<SemanticNode>,
  },
  Vardecl {
    var_type: Box<SemanticNode>,
    id: Box<SemanticNode>,
    const_index: Option<Box<SemanticNode>>,
  },
  VarIndex {
    index: Vec<SemanticNode>,
  },
  Terminal {
    value: Token
  }
}
