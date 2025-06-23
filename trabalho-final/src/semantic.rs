use std::error::Error;
use std::rc::Rc;
use std::io::Write;

use crate::code_attrs::CodeAttrs;
use crate::scope_stack::ScopeStack;
use crate::scope_stack::ScopeType;
use crate::scope_stack::SymbolEntry;
use crate::grammar::semantic_node::SemanticNodeData;
use crate::grammar::const_type::ConstType;
use crate::grammar::token_type::TokenType;

#[derive(Debug, Clone, PartialEq)]
pub struct SemanticNode {
  pub scopes: Rc<ScopeStack>,
  pub children: SemanticNodeData
}

#[derive(Debug, Clone, PartialEq)]
enum ReturnSem {
  Tipo(ConstType),
  Values(SemanticNode),
  TT(TokenType),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ArgSem {
  Nome(String),
}

impl SemanticNode {
  fn semantic_analysis(&self, arg: Option<&ArgSem>) -> Result<Option<ReturnSem>, Box<dyn Error>> {
    // Perform semantic analysis on the node
    match self.children.clone() {
      SemanticNodeData::Allocexpression {var_type, dimensions} => {
        var_type.semantic_analysis(arg)?;
        dimensions.semantic_analysis(arg)?;
        Ok(None)
      },
      SemanticNodeData::Atribstat {lvalue, value} => {
        lvalue.semantic_analysis(arg)?;
        value.semantic_analysis(arg)?;
        Ok(None)
      },
      SemanticNodeData::Atribstatevalue {expression, allocexpression, funccall} => {
        if let Some(expression) = expression {
          expression.semantic_analysis(arg)?;
        }
        if let Some(allocexpression) = allocexpression {
          allocexpression.semantic_analysis(arg)?;
        }
        if let Some(funccall) = funccall {
          funccall.semantic_analysis(arg)?;
        }
        Ok(None)
      },
      SemanticNodeData::Constant {value} => {
        // CONSTANT -> const_int
        //  CONSTANT.tipo = "int"
        return Ok(ReturnSem::Tipo(value.clone()).into());
      },
      SemanticNodeData::ConstIndex { index } => {
        for i in index.iter() {
          i.semantic_analysis(arg)?;
        }
        Ok(None)
      },
      SemanticNodeData::Elsestat {statement} => {
        // ELSESTAT_1 -> lbrace STATELIST rbrace
        //  STATELIST.scopes.push(ScopeType::Any)
        self.scopes.push_scope(ScopeType::Any);
        statement.semantic_analysis(arg)
      },
      SemanticNodeData::Expression {numexpression, op_expression, numexpression2} => {
        // EXPRESSION.children {
        //   [NUMEXPRESSION] => Ok,
        //   [NUMEXPRESSION, _, NUMEXPRESSION] => children[0].tipo == children[2].tipo
        //   _ => panic!()
        // }
        // EXPRESSION.tipo = children[0].tipo
        let tipo1 = numexpression.semantic_analysis(arg).unwrap().unwrap();
        if let Some(op_expression) = op_expression {
          if let Some(numexpression2) = numexpression2 {
            op_expression.semantic_analysis(arg)?;
            let tipo2 = numexpression2.semantic_analysis(arg).unwrap().unwrap();
            if tipo1 != tipo2 {
              return Err("Type mismatch in expression".into());
            }
          }
        }
        Ok(Some(tipo1))
      },
      // TODO (em lvalue e constant)
      SemanticNodeData::Factor {expression, lvalue, constant} => {
        // FACTOR -> CONSTANT
        //  FACTOR.tipo = CONSTANT.tipo

        // FACTOR -> LVALUE
        //  FACTOR.tipo = LVALUE.tipo

        // FACTOR -> lparenthesis NUMEXPRESSION rparenthesis
        //  FACTOR.tipo = NUMEXPRESSION.tipo
        if let Some(expression) = expression {
          expression.semantic_analysis(arg)
        } else if let Some(lvalue) = lvalue {
          lvalue.semantic_analysis(arg)
        } else if let Some(constant) = constant {
          constant.semantic_analysis(arg)
        } else {
          return Err("Factor must have either expression, lvalue or constant".into());
        }
      },
      SemanticNodeData::Forstat {init, condition, increment, body} => {
        init.semantic_analysis(arg)?;
        condition.semantic_analysis(arg)?;
        increment.semantic_analysis(arg)?;
        body.semantic_analysis(arg)?;
        // FORSTAT -> kw_for lparenthesis ATRIBSTAT semicolon EXPRESSION semicolon ATRIBSTAT rparenthesis lbrace STATELIST rbrace
        //  STATELIST.scopes.push(ScopeType::Loop)
        self.scopes.push_scope(ScopeType::Loop);
        Ok(None)
      },
      SemanticNodeData::Funccall {id, paramlistcall} => {
        id.semantic_analysis(arg)?;
        if let Some(paramlistcall) = paramlistcall {
          paramlistcall.semantic_analysis(arg)?;
        }
        Ok(None)
      },
      // TODO
      SemanticNodeData::Funcdef {func_id, paramlist, statelist} => {
        // FUNCDEF -> kw_def func_id lparenthesis PARAMLIST rparenthesis lbrace STATELIST rbrace
        // STATELIST.scopes.push(ScopeType::Function)
        self.scopes.push_scope(ScopeType::Function);
        // PARAMLIST.scopes.insert(PARAMLIST.values)


        func_id.semantic_analysis(arg)?;
        if let Some(paramlist) = paramlist {
          paramlist.semantic_analysis(arg)?;
        }
        statelist.semantic_analysis(arg)?;
        // FUNCDEF -> kw_def func_id lparenthesis PARAMLIST rparenthesis lbrace STATELIST rbrace
        // PARAMLIST.nome = func_id.val
        // PARAMLIST.values = []
        Ok(None)
      },
      SemanticNodeData::Funclist {funclist} => {
        for func in funclist.iter() {
          func.semantic_analysis(arg)?;
        }
        Ok(None)
      },
      SemanticNodeData::Ifstat {condition, then_branch, else_branch} => {
        condition.semantic_analysis(arg)?;
        then_branch.semantic_analysis(arg)?;
        if let Some(else_branch) = else_branch {
          else_branch.semantic_analysis(arg)?;
        }
        // IFSTAT -> kw_if lparenthesis EXPRESSION rparenthesis lbrace STATELIST rbrace ELSESTAT
        // STATELIST.scopes.push(ScopeType::Any)
        self.scopes.push_scope(ScopeType::Any);
        Ok(None)
      },
      // TODO (alinhar com factor)
      // FACTOR -> LVALUE
      //  FACTOR.tipo = LVALUE.tipo
      SemanticNodeData::Lvalue {id, var_index} => {
        id.semantic_analysis(arg)?;
        if let Some(var_index) = var_index {
          var_index.semantic_analysis(arg)?;
        }
        // LVALUE -> id VAR_INDEX
        //  LVALUE.tipo = LVALUE.scopes.get(id)
        return Ok(None);
      },
      // TODO (alinhar com expression)
      SemanticNodeData::Numexpression {term, op_numexpression, term2} => {
        term.semantic_analysis(arg)?;
        if let Some(op_numexpression) = op_numexpression {
          op_numexpression.semantic_analysis(arg)?;
        }
        if let Some(term2) = term2 {
          term2.semantic_analysis(arg)?;
        }
        // NUMEXPRESSION.children { 
        //   [TERM] => Ok,
        //   [TERM, _, TERM] => children[0].tipo == children[2].tipo,
        //   _ => panic!()
        // }
        // NUMEXPRESSION.tipo = children[0].tipo
        return Ok(None);
      },
      SemanticNodeData::OpExpression {op} => {
        Ok(Some(ReturnSem::TT(op.clone())))
      },
      SemanticNodeData::OpNumexpression {op} => {
        Ok(Some(ReturnSem::TT(op.clone())))
      },
      SemanticNodeData::OpTerm {op} => {
        Ok(Some(ReturnSem::TT(op.clone())))
      },
      // TODO
      SemanticNodeData::Paramlist { paramlist } => {
        for i in paramlist.iter() {
          i.semantic_analysis(arg)?;
        }
        // PPARAMLIST -> ''
        // PARAMLIST.scopes.insert(PARAMLIST.nome, [])
        let Some(ArgSem::Nome(nome)) = arg else {
          return Err("Expected name argument for Paramlist".into());
        };
        self.scopes.insert_symbol(nome.clone(), SymbolEntry {
          appearances: vec![],
          var_type: None,
          const_index: vec![],
        })?;

        // PARAMLIST -> (vartype id)+
        // PARAMLIST.scopes.insert(PARAMLIST.nome, PARAMLIST.values)
        
        Ok(None)
      },
      SemanticNodeData::Paramlistcall { paramlist } => {
        for i in paramlist.iter() {
          i.semantic_analysis(arg)?;
        }
        Ok(None)
      },
      SemanticNodeData::Printstat { expression } => {
        expression.semantic_analysis(arg)
      },
      SemanticNodeData::Program { funclist, statement } => {
        if let Some(funclist) = funclist {
          funclist.semantic_analysis(arg)
        } else if let Some(statement) = statement {
          statement.semantic_analysis(arg)
        } else {
          return Err("Program must have either funclist or statement".into());
        }
      },
      SemanticNodeData::Readstat { lvalue } => {
        lvalue.semantic_analysis(arg)
      },
      SemanticNodeData::Returnstat => {
        Ok(None)
      },
      SemanticNodeData::Statelist { statelist } => {
        for i in statelist.iter() {
          i.semantic_analysis(arg)?;
        }
        Ok(None)
      },
      SemanticNodeData::Statement {
        vardecl, 
        atribstat,
        ifstat,
        forstat,
        statelist,
        commandstat
      }
      => {
        if let Some(vardecl) = vardecl {
          vardecl.semantic_analysis(arg)?;
        }
        if let Some(atribstat) = atribstat {
          atribstat.semantic_analysis(arg)?;
        }
        if let Some(ifstat) = ifstat {
          ifstat.semantic_analysis(arg)?;
        }
        if let Some(forstat) = forstat {
          forstat.semantic_analysis(arg)?;
        }
        if let Some(statelist) = statelist {
          statelist.semantic_analysis(arg)?;
          // STATEMENT -> lbrace STATELIST rbrace
          // STATELIST.scopes.push(ScopeType::Any)
          self.scopes.push_scope(ScopeType::Any);
        }
        if let Some(commandstat) = commandstat {
          match commandstat.children {
            // STATEMENT -> RETURNSTAT semicolon
            // if !STATEMENT.scopes.contains(ScopeType::Function) { ERRO("Return keyword usada fora de uma função"); }
            SemanticNodeData::Returnstat => {
              if !self.scopes.contains(ScopeType::Function) {
                return Err("Return statement outside of function".into());
              }
            },
            // STATEMENT -> kw_break semicolon
            //  if !STATEMENT.scopes.contains(ScopeType::Loop) { ERRO("Break keyword usada fora de um laço de repetição"); }
            SemanticNodeData::Terminal { value } => {
              if value.token_type == TokenType::KwBreak {
                if !self.scopes.contains(ScopeType::Loop) {
                  return Err("Break statement outside of loop".into());
                }
              }
            },
            _ => {},
          }
          return commandstat.semantic_analysis(arg);
        }
        Err("Statement must have either vardecl, atribstat, ifstat, forstat, statelist or commandstat".into())
      },
      // TODO
      SemanticNodeData::Term { factor, op_term, factor2 } => {
        factor.semantic_analysis(arg)?;
        if let Some(op_term) = op_term {
          op_term.semantic_analysis(arg)?;
        }
        if let Some(factor2) = factor2 {
          factor2.semantic_analysis(arg)?;
        }
        // TERM.children {
        //     [UNARYEXPRESSION] => Ok,
        //     [UNARYEXPRESSION, _, UNARYEXPRESSION] => children[0].tipo == children[2].tipo,
        //     _ => panic!()
        // }
        // TERM.tipo = children[0].tipo
        return Ok(None);
      },
      // TODO
      SemanticNodeData::Unaryexpression { op, factor } => {
        if let Some(op) = op {
          op.semantic_analysis(arg)?;
        }
        factor.semantic_analysis(arg)?;
        // UNARYEXPRESSION.children {
        //   [FACTOR] => Ok,
        //   [_, Factor] => Ok,
        //   _ => panic!()
        // }
        // UNARYEXPRESSION.tipo = children[-1].tipo
        return Ok(None);
      },
      SemanticNodeData::Vardecl {var_type, id, const_index} => {
        var_type.semantic_analysis(arg)?;
        id.semantic_analysis(arg)?;
        if let Some(ci) = const_index {
          ci.semantic_analysis(arg)?;
        }
        // VARDECL -> vartype id
        // VARDECL.scopes.insert(id.val, vartype.val)
        let SemanticNodeData::Terminal {value: val1} = var_type.children else {
          return Err("Expected terminal node for var_type".into());
        };
        let var_type_val = val1.value.clone();
        let appearances = vec![(val1.line, val1.column)];
        let SemanticNodeData::Terminal {value: val2} = id.children else {
          return Err("Expected terminal node for id".into());
        };
        let ConstType::String(name) = val2.value.clone().unwrap() else {
          return Err("Expected string constant for id".into());
        };
        let mut dimensions = vec![];
        match const_index.clone() {
         Some(ci) => {
            let SemanticNodeData::ConstIndex {index} = ci.children else {
              return Err("Expected ConstIndex node for const_index".into());
            };
            for i in index.iter() {
              if let SemanticNodeData::Terminal {value} = i.children {
                if let Some(ConstType::Int(i)) = value.value {
                  dimensions.push(i as u32);
                } else {
                  return Err("Expected integer constant for const_index".into());
                }
              } else {
                return Err("Expected terminal node for const_index".into());
              }
            }
          },
          None => {
            dimensions.push(1);
          }
        }
        let entry = SymbolEntry {
          appearances,
          var_type: var_type_val,
          const_index: dimensions,
        };
        self.scopes.insert_symbol(
          name,
          entry
        )?;
        Ok(None)
      },
      SemanticNodeData::VarIndex {index} => {
        for i in index.iter() {
          i.semantic_analysis(arg)?;
        }
        Ok(None)
      }
      SemanticNodeData::Terminal { value: token } => {
        match token.token_type {
          TokenType::Rbrace => {
            // # Whenever vising a "}" node, close the previous scope
            // rbrace
            // rbrace.scopes.pop()
            let Some(_) = self.scopes.pop_scope() else {
              return Err("Unexpected '}' without matching '{'".into());
            };
            return Ok(Some(ReturnSem::TT(TokenType::Rbrace)));
          },
          TokenType::Eof => {
            // # Pop global scope
            // EOF
            // EOF.scopes.pop()
            if self.scopes.stack.len() == 1 {
              self.scopes.pop_scope();
              Ok(Some(ReturnSem::TT(TokenType::Eof)))
            } else {
              return Err("Unexpected EOF without matching '}'".into());
            }
          },
          TokenType::ConstInt => {
            // # CONSTANT -> const_int
            // #  CONSTANT.tipo = "int"
            if let Some(value) = &token.value {
              if let ConstType::Int(_) = value {
                Ok(Some(ReturnSem::Tipo(ConstType::Int(0))))
              } else {
                Err("Expected integer constant".into())
              }
            } else {
              Err("Expected value for const_int".into())
            }
          },
          TokenType::ConstFloat => {
            // # CONSTANT -> const_float
            // #  CONSTANT.tipo = "float"
            if let Some(value) = &token.value {
              if let ConstType::Float(_) = value {
                Ok(Some(ReturnSem::Tipo(ConstType::Float(0.0))))
              } else {
                Err("Expected float constant".into())
              }
            } else {
              Err("Expected value for const_float".into())
            }
          },
          TokenType::ConstString => {
            // # CONSTANT -> const_string
            // #  CONSTANT.tipo = "string"
            if let Some(value) = &token.value {
              if let ConstType::String(_) = value {
                Ok(Some(ReturnSem::Tipo(ConstType::String("".into()))))
              } else {
                Err("Expected string constant".into())
              }
            } else {
              Err("Expected value for const_string".into())
            }
          },
          // Comma | ConstNull | FuncId | Id
          //   | KwBreak | KwDef | KwElse | KwFor | KwIf | KwNew | KwPrint | KwRead
          //   | KwReturn | Lbrace | Lbracket | Lparenthesis | OpAssign | OpDivision
          //   | OpEq | OpGe | OpGt | OpLe | OpLt | OpMinus | OpModular | OpMult
          //   | OpNe | OpPlus | Rbracket | Rparenthesis | Semicolon | VarType
           _ => {
            Ok(None)
          }
        }
      },
    }
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
    self.root.semantic_analysis(None)?;
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