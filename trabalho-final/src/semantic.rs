use std::error::Error;
use std::rc::Rc;
use std::io::Write;

use crate::code_attrs::CodeAttrs;
use crate::scope_stack::ScopeStack;
use crate::scope_stack::ScopeType;
use crate::scope_stack::SymbolEntry;
use crate::grammar::semantic_node::SemanticNodeData;
use crate::grammar::const_type::ConstType;

#[derive(Debug, Clone, PartialEq)]
pub struct SemanticNode {
  pub scopes: Rc<ScopeStack>,
  pub children: SemanticNodeData

enum ReturnSem {
  Tipo(ConstType),
  Values()
}

impl SemanticNode {
  fn semantic_analysis(&self) -> Result<Option(ReturnSem), Box<dyn Error>> {
    // Perform semantic analysis on the node
    match self.children {
      SemanticNodeData::Allocexpression {var_type, dimensions} => {
        var_type.semantic_analysis()?;
        dimensions.semantic_analysis()?;
        Ok(None)
      },
      SemanticNodeData::Atribstat {lvalue, value} => {
        lvalue.semantic_analysis()?;
        value.semantic_analysis()?;
        Ok(None)
      },
      SemanticNodeData::Atribstatevalue {expression, allocexpression, funccall} => {
        if let Some(expression) = expression {
          expression.semantic_analysis()?;
        }
        if let Some(allocexpression) = allocexpression {
          allocexpression.semantic_analysis()?;
        }
        if let Some(funccall) = funccall {
          funccall.semantic_analysis()?;
        }
        Ok(None)
      },
      SemanticNodeData::Constant {value} => {
        // CONSTANT -> const_int
        //  CONSTANT.tipo = "int"
        value.semantic_analysis()
      },
      SemanticNodeData::ConstIndex {index} => {
        index.iter().map(|i| i.semantic_analysis())?;
        Ok(None)
      },
      SemanticNodeData::Elsestat {statement} => {
        // ELSESTAT_1 -> lbrace STATELIST rbrace
        //  STATELIST.scopes.push(ScopeType::Any)
        self.scopes.push_scope(ScopeType::Any);
        statement.semantic_analysis()
      },
      // TODO
      SemanticNodeData::Expression {numexpression, op_expression, numexpression2} => {
        numexpression.semantic_analysis()?;
        if let Some(op_expression) = op_expression {
          op_expression.semantic_analysis()?;
        }
        if let Some(numexpression2) = numexpression2 {
          numexpression2.semantic_analysis()?;
        }
        // EXPRESSION.children {
        //   [NUMEXPRESSION] => Ok,
        //   [NUMEXPRESSION, _, NUMEXPRESSION] => children[0].tipo == children[2].tipo
        //   _ => panic!()
        // }
        // EXPRESSION.tipo = children[0].tipo
      },
      // TODO
      SemanticNodeData::Factor {expression, lvalue, constant} => {
        if let Some(expression) = expression {
          expression.semantic_analysis()?;
        }
        if let Some(lvalue) = lvalue {
          lvalue.semantic_analysis()?;
        }
        if let Some(constant) = constant {
          constant.semantic_analysis()?;
        }
        // FACTOR -> CONSTANT
        //  FACTOR.tipo = CONSTANT.tipo

        // FACTOR -> LVALUE
        //  FACTOR.tipo = LVALUE.tipo

        // FACTOR -> lparenthesis NUMEXPRESSION rparenthesis
        //  FACTOR.tipo = NUMEXPRESSION.tipo
      },
      SemanticNodeData::Forstat {init, condition, increment, body} => {
        init.semantic_analysis()?;
        condition.semantic_analysis()?;
        increment.semantic_analysis()?;
        body.semantic_analysis()?;
        // FORSTAT -> kw_for lparenthesis ATRIBSTAT semicolon EXPRESSION semicolon ATRIBSTAT rparenthesis lbrace STATELIST rbrace
        //  STATELIST.scopes.push(ScopeType::Loop)
        self.scopes.push_scope(ScopeType::Loop);
        Ok(None)
      },
      SemanticNodeData::Funccall {id, paramlistcall} => {
        id.semantic_analysis()?;
        if let Some(paramlistcall) = paramlistcall {
          paramlistcall.semantic_analysis()?;
        }
        Ok(None)
      },
      // TODO
      SemanticNodeData::Funcdef {func_id, paramlist, statelist} => {
        // FUNCDEF -> kw_def func_id lparenthesis PARAMLIST rparenthesis lbrace STATELIST rbrace
        // STATELIST.scopes.push(ScopeType::Function)
        self.scopes.push_scope(ScopeType::Function);
        // PARAMLIST.scopes.insert(PARAMLIST.values)

        func_id.semantic_analysis()?;
        if let Some(paramlist) = paramlist {
          paramlist.semantic_analysis()?;
        }
        statelist.semantic_analysis()?;
        // FUNCDEF -> kw_def func_id lparenthesis PARAMLIST rparenthesis lbrace STATELIST rbrace
        // PARAMLIST.nome = func_id.val
        // PARAMLIST.values = []
        Ok(None)
      },
      SemanticNodeData::Funclist {funclist} => {
        funclist.iter().map(|i| i.semantic_analysis())?;
        Ok(None)
      },
      SemanticNodeData::Ifstat {condition, then_branch, else_branch} => {
        condition.semantic_analysis()?;
        then_branch.semantic_analysis()?;
        if let Some(else_branch) = else_branch {
          else_branch.semantic_analysis()?;
        }
        // IFSTAT -> kw_if lparenthesis EXPRESSION rparenthesis lbrace STATELIST rbrace ELSESTAT
        // STATELIST.scopes.push(ScopeType::Any)
        self.scopes.push_scope(ScopeType::Any);
        Ok(None)
      },
      // TODO
      SemanticNodeData::Lvalue {id, var_index} => {
        id.semantic_analysis()?;
        if let Some(var_index) = var_index {
          var_index.semantic_analysis()?;
        }
        // LVALUE -> id VAR_INDEX
        //  LVALUE.tipo = LVALUE.scopes.get(id)
      },
      // TODO
      SemanticNodeData::Numexpression {term, op_expression, term2} => {
        term.semantic_analysis()?;
        if let Some(op_expression) = op_expression {
          op_expression.semantic_analysis()?;
        }
        if let Some(term2) = term2 {
          term2.semantic_analysis()?;
        }
        // NUMEXPRESSION.children { 
        //   [TERM] => Ok,
        //   [TERM, _, TERM] => children[0].tipo == children[2].tipo,
        //   _ => panic!()
        // }
        // NUMEXPRESSION.tipo = children[0].tipo
      },
      SemanticNodeData::OpExpression {op} => {
        op.semantic_analysis()
      },
      SemanticNodeData::OpNumexpression {op} => {
        op.semantic_analysis()
      },
      SemanticNodeData::OpTerm {op} => {
        op.semantic_analysis()
      },
      // TODO
      SemanticNodeData::Paramlist { paramlist } => {
        paramlist.iter().map(|i| i.semantic_analysis())?;
        // PPARAMLIST -> ''
        // PARAMLIST.scopes.insert(PARAMLIST.nome, [])

        // PARAMLIST -> (vartype id)+
        // PARAMLIST.scopes.insert(PARAMLIST.nome, PARAMLIST.values)
        
        Ok(None)
      },
      SemanticNodeData::Paramlistcall { paramlist } => {
        paramlist.iter().map(|i| i.semantic_analysis())?;
        Ok(None)
      },
      SemanticNodeData::Printstat { expression } => {
        expression.semantic_analysis()?
      },
      SemanticNodeData::Program { funclist, statement } => {
        if let Some(funclist) = funclist {
          funclist.semantic_analysis()
        } else if let Some(statement) = statement {
          statement.semantic_analysis()?
        }
      },
      SemanticNodeData::Readstat { lvalue } => {
        lvalue.semantic_analysis()?
      },
      SemanticNodeData::Returnstat {} => {
        Ok(None)
      },
      SemanticNodeData::Statelist { statements } => {
        statements.iter().map(|i| i.semantic_analysis())?;
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
          vardecl.semantic_analysis()?;
        }
        if let Some(atribstat) = atribstat {
          atribstat.semantic_analysis()?;
        }
        if let Some(ifstat) = ifstat {
          ifstat.semantic_analysis()?;
        }
        if let Some(forstat) = forstat {
          forstat.semantic_analysis()?;
        }
        if let Some(statelist) = statelist {
          statelist.semantic_analysis()?;
          // STATEMENT -> lbrace STATELIST rbrace
          // STATELIST.scopes.push(ScopeType::Any)
          self.scopes.push_scope(ScopeType::Any);
        }
        if let Some(commandstat) = commandstat {
          commandstat.semantic_analysis()?;
          match commandstat {
            // STATEMENT -> RETURNSTAT semicolon
            // if !STATEMENT.scopes.contains(ScopeType::Function) { ERRO("Return keyword usada fora de uma função"); }
            SemanticNodeData::Returnstat {} => {
              if !self.scopes.contains(ScopeType::Function) {
                return Err("Return statement outside of function".into());
              }
            },
            // STATEMENT -> kw_break semicolon
            //  if !STATEMENT.scopes.contains(ScopeType::Loop) { ERRO("Break keyword usada fora de um laço de repetição"); }
            SemanticNodeData::Terminal { token } => {
              if token.token_type == TokenType::KwBreak {
                if !self.scopes.contains(ScopeType::Loop) {
                  return Err("Break statement outside of loop".into());
                }
              },
            }
          }
        },
      },
      // TODO
      SemanticNodeData::Term { factor, op_term, factor2 } => {
        factor.semantic_analysis()?;
        if let Some(op_term) = op_term {
          op_term.semantic_analysis()?;
        }
        if let Some(factor2) = factor2 {
          factor2.semantic_analysis()?;
        }
        // TERM.children {
        //     [UNARYEXPRESSION] => Ok,
        //     [UNARYEXPRESSION, _, UNARYEXPRESSION] => children[0].tipo == children[2].tipo,
        //     _ => panic!()
        // }
        // TERM.tipo = children[0].tipo
      },
      // TODO
      SemanticNodeData::Unaryexpression { op, factor } => {
        if let Some(op) = op {
          op.semantic_analysis()?;
        }
        factor.semantic_analysis()?;
        // UNARYEXPRESSION.children {
        //   [FACTOR] => Ok,
        //   [_, Factor] => Ok,
        //   _ => panic!()
        // }
        // UNARYEXPRESSION.tipo = children[-1].tipo
      },
      SemanticNodeData::Vardecl {var_type, id, const_index} => {
        var_type.semantic_analysis()?;
        id.semantic_analysis()?;
        if let Some(const_index) = const_index {
          const_index.semantic_analysis()?;
        }
        // VARDECL -> vartype id
        // VARDECL.scopes.insert(id.val, vartype.val)
        let SemanticNodeData::Terminal {value} = var_type.children else {
          return Err("Expected terminal node for var_type".into());
        };
        var_type_val = value.value.unwrap().clone();
        appearances = vec![(value.line, value.column)];
        name = id.value.value.clone().unwrap();
        dimensions = vec![];
        match const_index {
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
        entry = SymbolEntry {
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
        index.iter().map(|i| i.semantic_analysis())?;
        Ok(None)
      }
      SemanticNodeData::Terminal { token } => {
        match token.token_type {
          TokenType::Rbrace => {
            // # Whenever vising a "}" node, close the previous scope
            // rbrace
            // rbrace.scopes.pop()
            let Some(scope) = self.scopes.pop_scope() else {
              return Err("Unexpected '}' without matching '{'".into());
            };
          },
          TokenType::Eof => {
            // # Pop global scope
            // EOF
            // EOF.scopes.pop()
            if self.scopes.stack.len() == 1 {
              self.scopes.pop_scope();
            } else {
              return Err("Unexpected EOF without matching '}'".into());
            }
          },
          ConstInt => {
            // # CONSTANT -> const_int
            // #  CONSTANT.tipo = "int"
            if let Some(value) = &token.value {
              if let ConstType::Int(_) = value {
                Ok(Some(ReturnSem::Tipo(ConstType::Int)))
              } else {
                Err("Expected integer constant".into())
              }
            } else {
              Err("Expected value for const_int".into())
            }
          },
          (
            Comma | ConstFloat | ConstNull | ConstString | FuncId | Id
            | KwBreak | KwDef | KwElse | KwFor | KwIf | KwNew | KwPrint | KwRead
            | KwReturn | Lbrace | Lbracket | Lparenthesis | OpAssign | OpDivision
            | OpEq | OpGe | OpGt | OpLe | OpLt | OpMinus | OpModular | OpMult
            | OpNe | OpPlus | Rbracket | Rparenthesis | Semicolon | VarType
          ) => {}
        }
      },
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