use crate::grammar::{token_type::TokenType, const_type::ConstType};
use crate::token::Token;
use crate::semantic::SemanticNode;

#[derive(Debug, Clone, PartialEq)]
pub enum SemanticNodeData {
  // ALLOCEXPRESSION -> vartype VARINDEX
  Allocexpression {
    var_type: Box<SemanticNode>,
    dimensions: Box<SemanticNode>,
  },
  // ATRIBSTAT -> LVALUE value
  Atribstat {
    lvalue: Box<SemanticNode>,
    value: Box<SemanticNode>,
  },
  // ATRIBSTATEVALUE -> EXPRESSION 
  // ATRIBSTATEVALUE -> ALLOCEXPRESSION
  // ATRIBSTATEVALUE -> FUNCCALL
  Atribstatevalue {
    expression: Option<Box<SemanticNode>>,
    allocexpression: Option<Box<SemanticNode>>,
    funccall: Option<Box<SemanticNode>>,
  },
  Constant {
    value: ConstType
  },
  // CONSTINDEX -> [CONSTANT1, CONSTANT2, CONSTANT3...]
  ConstIndex {
    index: Vec<SemanticNode>,
  },
  // ELSESTAT -> STATEMENT
  Elsestat {
    statement: Box<SemanticNode>,
  },
  // EXPRESSION -> NUMEXPRESSION op_expression numexpression2 
  // EXPRESSION -> NUMEXPRESSION 
  Expression {
    numexpression: Box<SemanticNode>,
    op_expression: Option<Box<SemanticNode>>,
    numexpression2: Option<Box<SemanticNode>>,
  },
  // FACTOR -> EXPRESSION
  // FACTOR -> LVALUE
  // FACTOR -> constant
  Factor {
    expression: Option<Box<SemanticNode>>,
    lvalue: Option<Box<SemanticNode>>,
    constant: Option<Box<SemanticNode>>,
  },
  // FORSTAT -> ATRIBSTAT EXPRESSION ATRIBSTAT STATELIST
  Forstat {
    init: Box<SemanticNode>,
    condition: Box<SemanticNode>,
    increment: Box<SemanticNode>,
    body: Box<SemanticNode>,
  },
  // FUNCCALL -> id
  // FUNCCALL -> id PARAMLISTCALL
  Funccall {
    id: Box<SemanticNode>,
    paramlistcall: Option<Box<SemanticNode>>,
  },
  // FUNCDEF -> func_id PARAMLIST STATELIST
  // FUNCDEF -> func_id STATELIST
  Funcdef {
    func_id: Box<SemanticNode>,
    paramlist: Option<Box<SemanticNode>>,
    statelist: Box<SemanticNode>,
  },
  // FUNCLIST -> [FUNCDEF1, FUNCDEF2, FUNCDEF3...]
  Funclist {
    funclist: Vec<SemanticNode>,
  },
  // IFSTAT -> EXPRESSION STATELIST
  // IFSTAT -> EXPRESSION STATELIST STATELIST
  Ifstat {
    condition: Box<SemanticNode>,
    then_branch: Box<SemanticNode>,
    else_branch: Option<Box<SemanticNode>>,
  },
  // LVALUE -> id
  // LVALUE -> id VARINDEX
  Lvalue {
    id: Box<SemanticNode>,
    var_index: Option<Box<SemanticNode>>,
  },
  // NUMEXPRESSION -> term
  // NUMEXPRESSION -> term op_numexpression term2
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
  // PARAMLIST -> [vartype1, id1, vartype2, id2, vartype3, id3...]
  Paramlist {
    paramlist: Vec<SemanticNode>,
  },
  // PARAMLISTCALL -> [id1, id2, id3...]
  Paramlistcall {
    paramlist: Vec<SemanticNode>,
  },
  // PRINTSTAT -> EXPRESSION
  Printstat {
    expression: Box<SemanticNode>,
  },
  // PROGRAM -> FUNCLIST
  // PROGRAM -> STATEMENT
  Program {
    funclist: Option<Box<SemanticNode>>,
    statement: Option<Box<SemanticNode>>,
  },
  // READSTAT -> LVALUE
  Readstat {
    lvalue: Box<SemanticNode>,
  },
  Returnstat {
    token: Token,
  },
  // STATELIST -> [STATEMENT1, STATEMENT2, STATEMENT3...]
  Statelist {
    statelist: Vec<SemanticNode>,
  },
  // STATEMENT -> VARDECL 
  // STATEMENT -> ATRIBSTAT
  // STATEMENT -> IFSTAT
  // STATEMENT -> FORSTAT
  // STATEMENT -> STATELIST
  // STATEMENT -> PRINTSTAT
  // STATEMENT -> READSTAT
  // STATEMENT -> RETURNSTAT
  Statement {
    vardecl: Option<Box<SemanticNode>>,
    atribstat: Option<Box<SemanticNode>>,
    ifstat: Option<Box<SemanticNode>>,
    forstat: Option<Box<SemanticNode>>,
    statelist: Option<Box<SemanticNode>>,
    commandstat: Option<Box<SemanticNode>>,
  },
  // TERM -> UNARYEXPRESSION
  // TERM -> UNARYEXPRESSION op_term UNARYEXPRESSION
  Term {
    unaryexpression: Box<SemanticNode>,
    op_term: Option<Box<SemanticNode>>,
    unaryexpression2: Option<Box<SemanticNode>>,
  },
  // UNARYEXPRESSION -> FACTOR
  // UNARYEXPRESSION -> op FACTOR
  Unaryexpression {
    op: Option<Box<SemanticNode>>,
    factor: Box<SemanticNode>,
  },
  //VARDECL -> var_type id
  //VARDECL -> var_type id CONSTINDEX
  Vardecl {
    var_type: Box<SemanticNode>,
    id: Box<SemanticNode>,
    const_index: Option<Box<SemanticNode>>,
  },
  // VARINDEX -> [NUMEXPRESSION1, NUMEXPRESSION2, NUMEXPRESSION3]
  VarIndex {
    index: Vec<SemanticNode>,
  },
  Terminal {
    value: Token
  }
}
