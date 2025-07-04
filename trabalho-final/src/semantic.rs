use core::panic;
use std::error::Error;
use crate::code_attrs::CodeAttrs;
use crate::scope_stack::ScopeStack;
use crate::scope_stack::ScopeType;
use crate::scope_stack::SymbolEntry;
use crate::grammar::semantic_node::SemanticNodeData;
use crate::grammar::const_type::{ConstType, VarType};
use crate::grammar::token_type::TokenType;
use crate::expression::*;

#[derive(Debug, Clone, PartialEq)]
pub struct SemanticNode {
  pub children: SemanticNodeData
}

#[derive(Debug, Clone, PartialEq)]
enum ReturnSem {
  Dado {
    tipo: VarType,
    pos: Option<(usize, usize)>,
  },
  TT(TokenType),
}

impl SemanticNode {
  fn semantic_analysis(&self, scopes: &mut ScopeStack) -> Result<Option<ReturnSem>, Box<dyn Error>> {
    match self.children.clone() {
      SemanticNodeData::Allocexpression {var_type, dimensions} => {
        let tipo = var_type.semantic_analysis(scopes)?;
        dimensions.semantic_analysis(scopes)?;
        Ok(tipo)
      },
      SemanticNodeData::Atribstat {lvalue, value} => {
        // ATRIBSTAT -> LVALUE ATRIBSTATVALUE
        //  if LVALUE.tipo != ATRIBSTATVALUE.tipo: ERRO 

        // get lvalue id
        // 
        let SemanticNodeData::Lvalue { id, var_index } = lvalue.children else { panic!() };
        let SemanticNodeData::Terminal { value: id_token } = id.children else { panic!() };
        let ConstType::String(id_name) = id_token.value.clone().unwrap() else { panic!() };

        // Insert id appearance in the current scope
        scopes.count_appearance(&id_name, id_token.line, id_token.column)?;

        // Check if the variable is declared in the current scope
        let Some(symbol_entry) = scopes.get_symbol(&id_name) else {
          return Err(format!("Erro semântico: variável '{}' não declarada no escopo atual na linha {}, coluna {}", id_name, id_token.line, id_token.column).into());
        };

        let Some(ReturnSem::Dado{tipo: value_type, pos: Some((value_line, value_column))}) = value.semantic_analysis(scopes)? else { panic!(); };
        if value_type != symbol_entry.var_type[0] {
          return Err(format!("Erro semântico: tipo incompatível na atribuição de '{}' na linha {} coluna {}", id_name, value_line, value_column).into());
        }
        // Check if the variable index is valid
        // LVALUE -> id VAR_INDEX
        if let Some(var_index) = var_index {
          let SemanticNodeData::VarIndex { index } = var_index.children else { panic!("{:?}", var_index.children) };
          for child in index.iter() {
            let tipo = child.semantic_analysis(scopes)?;
            if let Some(ReturnSem::Dado { tipo, pos: Some(index_pos) }) = tipo {
              if tipo != VarType::Int {
                return Err(format!("Erro semântico: índice de variável deve ser do tipo 'int', encontrado '{:?}' na linha {}, coluna {}", tipo, index_pos.0, index_pos.1).into());
              }
            } else {
              panic!(); 
            }
          }
        }
        Ok(None)
      },
      SemanticNodeData::Atribstatevalue {expression, allocexpression, funccall} => {

        // ATRIBSTATEVALUE -> EXPRESSION
        // ATRIBSTATEVALUE.tipo = EXPRESSION.tipo
        if let Some(expression) = expression {
          return Ok(expression.semantic_analysis(scopes)?);
        }
        
        // ATRIBSTATEVALUE -> ALLOCEXPRESSION
        // ATRIBSTATEVALUE.tipo -> ALLOCEXPRESSION.tipo
        if let Some(allocexpression) = allocexpression {
          return Ok(allocexpression.semantic_analysis(scopes)?);
        }

        // ATRIBSTATEVALUE -> FUNCCALL
        // ATRIBSTATEVALUE.tipo -> FUNCCALL.tipo
        if let Some(funccall) = funccall {
          return Ok(funccall.semantic_analysis(scopes)?);
        }
        Ok(None)
      },
      SemanticNodeData::Constant {value, line, column} => {
        // CONSTANT -> const_int
        //  CONSTANT.tipo = "int"
        return Ok(Some(ReturnSem::Dado{ tipo: value.get_type(), pos: Some((line, column)) }));
      },
      SemanticNodeData::ConstIndex { index } => {
        // CONSTINDEX -> [CONSTANT1, CONSTANT2, CONSTANT3...]
        for i in index.iter() {
          i.semantic_analysis(scopes)?;
        }
        Ok(None)
      },
      SemanticNodeData::Elsestat {statement} => {
        // ELSESTAT_1 -> lbrace STATELIST rbrace
        //  STATELIST.scopes.push(ScopeType::Any)
        scopes.push_scope(ScopeType::Else);
        let result = statement.semantic_analysis(scopes);
        scopes.pop_scope();
        result
      },
      SemanticNodeData::Expression {numexpression, numexpression2, ..} => {
        // EXPRESSION.children {
        //   [NUMEXPRESSION] => Ok,
        //   [NUMEXPRESSION, _, NUMEXPRESSION] => children[0].tipo == children[2].tipo
        //   _ => panic!()
        // }
        // EXPRESSION.tipo = children[0].tipo
        let ReturnSem::Dado { tipo: tipo1, pos: exp1_pos} = numexpression.semantic_analysis(scopes)?.unwrap() else { panic!(); };
        if let Some(numexpression2) = numexpression2 {
          let ReturnSem::Dado { tipo: tipo2, pos: exp2_pos} = numexpression2.semantic_analysis(scopes)?.unwrap() else { panic!(); };
          let exp2_pos = exp2_pos.unwrap_or((0, 0));
          if tipo1 != tipo2 {
            return Err(format!("Erro semântico: Tipos incompátiveis na expressão na linha {} coluna {}", exp2_pos.0, exp2_pos.1).into());
          } else {
            // Sempre que uma expressão possui uma operação (de comparação), o valor retornado será uma int
            // falso: 0
            // verdadeiro: 1
            return Ok(Some(ReturnSem::Dado { tipo: VarType::Int, pos: exp1_pos }));
          }
        }
        Ok(Some(ReturnSem::Dado { tipo: tipo1, pos: exp1_pos }))
      },
      SemanticNodeData::Factor {expression, lvalue, constant} => {
        // FACTOR -> CONSTANT
        //  FACTOR.tipo = CONSTANT.tipo

        // FACTOR -> LVALUE
        //  FACTOR.tipo = LVALUE.tipo

        // FACTOR -> lparenthesis NUMEXPRESSION rparenthesis
        //  FACTOR.tipo = NUMEXPRESSION.tipo
        if let Some(expression) = expression { return expression.semantic_analysis(scopes); }
        if let Some(lvalue) = lvalue { return lvalue.semantic_analysis(scopes); }
        if let Some(constant) = constant { return constant.semantic_analysis(scopes); }
        panic!();
      },
      SemanticNodeData::Forstat {init, condition, increment, body} => {
        // FORSTAT -> kw_for lparenthesis ATRIBSTAT semicolon EXPRESSION semicolon ATRIBSTAT rparenthesis lbrace STATELIST rbrace
        //  STATELIST.scopes.push(ScopeType::Loop)
        // Escopo das operações do laço (atribstat, expression, atribstat)
        scopes.push_scope(ScopeType::LoopInit);
        init.semantic_analysis(scopes)?;
        condition.semantic_analysis(scopes)?;
        increment.semantic_analysis(scopes)?;

        // Escopo do corpo de execução do laço
        scopes.push_scope(ScopeType::Loop);
        body.semantic_analysis(scopes)?;
        scopes.pop_scope(); // Corpo
        scopes.pop_scope(); // Operações do laço
        Ok(None)
      },
      SemanticNodeData::Funccall {id, paramlistcall} => {
        // FUNCCALL -> id
        // FUNCCALL -> id PARAMLISTCALL
        let SemanticNodeData::Terminal { value } = id.children else { panic!() };
        let (ConstType::String(func_id), func_line, func_col) = (value.value.clone().unwrap(), value.line, value.column) else { panic!() };
        let Some(func_types) = scopes.get_symbol(&func_id) else { return Err("Erro Semântico: função não definida nesse escopo".into()); };
        
        let mut called_types: Vec<VarType> = vec![];
        // verificar tipo (id PARAMLISTCALL)
        match paramlistcall {
          None => {},
          Some(paramlistcall) => {
            let SemanticNodeData::Paramlistcall { paramlist } = &paramlistcall.children else { panic!(); };
            for param in paramlist.iter() {
              let SemanticNodeData::Terminal { value } = &param.children else { panic!(); };
              let ConstType::String(param_value) = value.value.clone().unwrap() else { panic!(); };
              let id_type = scopes.get_symbol(&param_value);
              match id_type {
                Some(symbol_entry) => {
                  let var_type = symbol_entry.var_type.clone();
                  if var_type.len() > 1 {
                    // Esse erro é tratado na análise sintática
                    return Err(format!("Erro semântico: função '{}' não pode ser passada como parâmetro de outra função", param_value).into());
                  }
                  called_types.push(var_type.first().unwrap().clone());
                },
                None => return Err(format!("Erro semântico: variável '{}' não definida no escopo atual na linha {}, coluna {}", param_value, value.line, value.column).into()),
              }
              // Count the appearance of the parameter
              scopes.count_appearance(&param_value, value.line, value.column)?;
            }

          }
        }

        // Check if called_types matches func_types
        if func_types.var_type != called_types {
          return Err(format!("Erro semântico: tipos de parâmetros incompatíveis na chamada da função '{}' na linha {}, coluna {}", func_id, func_line, func_col).into());
        }
        // Count the appearance of the function
        scopes.count_appearance(&func_id, value.line, value.column)?;
        Ok(Some(ReturnSem::Dado { tipo: VarType::Int, pos: Some((func_line, func_col)) }))  // Assuming all function calls return an int
      },
      SemanticNodeData::Funcdef {func_id, paramlist, statelist} => {
        // Get function name
        // PARAMLIST.inh = func_id
        let SemanticNodeData::Terminal { value } = func_id.children else { panic!() };
        let ConstType::String(func_id) = value.value.clone().unwrap() else { panic!(); };
        
        // Read function parameters
        // PARAMLIST
        let mut func_params_types: Vec<VarType> = vec![];
        let mut func_params: Vec<(VarType, String, (usize, usize))> = vec![];
        let mut prev_param = None;
        // PARAMLIST -> (vartype id)+
        //   PARAMLIST.tipos = [vartype1, id1, vartype2, id2 ...]
        if let Some(paramlist) = &paramlist {
          let SemanticNodeData::Paramlist { paramlist } = &paramlist.children else { panic!(); };
          for child in paramlist.iter() {
            let SemanticNodeData::Terminal { value: token } = &child.children else { panic!(); };
            match token.token_type {
              TokenType::VarType => {
                // Get the type of the parameter
                let var_type = token.value.as_ref().unwrap().get_keyword_type();
                func_params_types.push(var_type.clone());
                prev_param = Some(var_type);
              },
              TokenType::Id => {
                // Get the name of the parameter
                let ConstType::String(func_name) = token.clone().value.unwrap().clone() else { panic!(); };
                func_params.push((prev_param.clone().unwrap(), func_name, (token.line, token.column)));
              },
              _ => panic!(),
            }
          }
        }
        else {
          // PARAMLIST -> ''
          //  PARAMLIST.tipos = []
        }

        // insert(PARAMLIST.inh, PARAMLIST.tipos)
        let entry = SymbolEntry {
          appearances: vec![(value.line, value.column)],
          var_type: func_params_types,
          const_index: vec![],
        };
        scopes.insert_symbol(func_id.clone(), entry)?;
        // Push a new scope for the function body
        // And insert the function parameters into the scope
        scopes.push_scope(ScopeType::Function);
        for (param_type, param_name, pos) in func_params {
          let entry = SymbolEntry {
            appearances: vec![pos],
            var_type: vec![param_type],
            const_index: vec![],
          };
          scopes.insert_symbol(param_name, entry)?;
        }

        // Analyze the function body
        statelist.semantic_analysis(scopes)?;
        scopes.pop_scope(); // Pop the function scope
        Ok(None)
      },
      SemanticNodeData::Funclist {funclist} => {
        for func in funclist.iter() {
          func.semantic_analysis(scopes)?;
        }
        Ok(None)
      },
      SemanticNodeData::Ifstat {condition, then_branch, else_branch} => {
        condition.semantic_analysis(scopes)?;
        scopes.push_scope(ScopeType::If);
        then_branch.semantic_analysis(scopes)?;
        scopes.pop_scope();
        if let Some(else_branch) = else_branch {
          else_branch.semantic_analysis(scopes)?;
        }
        // IFSTAT -> kw_if lparenthesis EXPRESSION rparenthesis lbrace STATELIST rbrace ELSESTAT
        // STATELIST.scopes.push(ScopeType::Any)
        Ok(None)
      },
      SemanticNodeData::Lvalue {id, var_index} => {
        let tipo = id.semantic_analysis(scopes)?.unwrap();
        if let Some(var_index) = var_index {
          var_index.semantic_analysis(scopes)?;
        }
        // LVALUE -> id VAR_INDEX
        //  LVALUE.tipo = LVALUE.scopes.get(id)
        return Ok(Some(tipo));
      },
      SemanticNodeData::Numexpression {term, op_numexpression, term2} => {
        let ReturnSem::Dado { tipo: tipo1, pos: exp1_pos } = term.semantic_analysis(scopes)?.unwrap() else { panic!(); };
        if let Some(op_numexpression) = op_numexpression {
          op_numexpression.semantic_analysis(scopes)?;
        }
        if let Some(term2) = term2 {
          let ReturnSem::Dado { tipo: tipo2, pos: exp2_pos } = term2.semantic_analysis(scopes)?.unwrap() else { panic!(); };
          let exp2_pos = exp2_pos.unwrap_or((0, 0));
          if tipo1 != tipo2 {
            return Err(format!("Erro semântico: tipos incompatíveis na expressão numérica na linha {} coluna {}", exp2_pos.0, exp2_pos.1).into());
          }
        }
        // NUMEXPRESSION.children { 
        //   [TERM] => Ok,
        //   [TERM, _, TERM] => children[0].tipo == children[2].tipo,
        //   _ => panic!()
        // }
        // NUMEXPRESSION.tipo = children[0].tipo
        return Ok(Some(ReturnSem::Dado { tipo: tipo1, pos: exp1_pos }));
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
      SemanticNodeData::Paramlist {..} => {
        panic!();
      },
      SemanticNodeData::Paramlistcall { paramlist } => {
        for i in paramlist.iter() {
          i.semantic_analysis(scopes)?;
        }
        Ok(None)
      },
      SemanticNodeData::Printstat { expression } => {
        expression.semantic_analysis(scopes)
      },
      SemanticNodeData::Program { funclist, statement } => {
        if let Some(funclist) = funclist { return funclist.semantic_analysis(scopes); }
        if let Some(statement) = statement { return statement.semantic_analysis(scopes); }
        panic!();
      },
      SemanticNodeData::Readstat { lvalue } => {
        // get value of lvalue
        let SemanticNodeData::Lvalue { id, .. } = lvalue.children else { panic!() };
        let SemanticNodeData::Terminal { value: id_token } = id.children else { panic!() };
        let ConstType::String(id_name) = id_token.value.clone().unwrap() else { panic!(); };
        // Count the appearance of the variable
        scopes.count_appearance(&id_name, id_token.line, id_token.column)?;
        // Check variable type
        let Some(symbol_entry) = scopes.get_symbol(&id_name) else {
          return Err(format!("Erro semântico: variável '{}' não declarada no escopo atual na linha {} coluna {}", id_name, id_token.line, id_token.column).into());
        };
        if symbol_entry.var_type[0] != VarType::String {
          return Err(format!("Erro semântico: comando Read deve atribuir valor a uma variável de tipo string, mas tipo {:?} foi encontrado na linha {} coluna {}", symbol_entry.var_type[0], id_token.line, id_token.column).into());
        }
        Ok(None)
      },
      SemanticNodeData::Returnstat { .. } => {
        Ok(None)
      },
      SemanticNodeData::Statelist { statelist } => {
        for statement in statelist.iter() {
          statement.semantic_analysis(scopes)?;
        }
        Ok(None)
      },
      SemanticNodeData::Statement { vardecl,  atribstat, ifstat, forstat, statelist, commandstat } => {
        if let Some(vardecl) = vardecl { return vardecl.semantic_analysis(scopes); }
        if let Some(atribstat) = atribstat { return atribstat.semantic_analysis(scopes); }
        if let Some(ifstat) = ifstat { return ifstat.semantic_analysis(scopes); }
        if let Some(forstat) = forstat { return forstat.semantic_analysis(scopes); }
        if let Some(statelist) = statelist {
          scopes.push_scope(ScopeType::Any);
          statelist.semantic_analysis(scopes)?;
          scopes.pop_scope();
          return Ok(None);
        }
        if let Some(commandstat) = commandstat {
          match &commandstat.children {
            SemanticNodeData::Returnstat { token } => {
              if !scopes.contains(ScopeType::Function) { return Err(format!("Erro semântico: Comando 'return' fora de um laço de repetição na linha {} coluna {}", token.line, token.column).into()); }
            },
            // STATEMENT -> kw_break semicolon
            //  if !STATEMENT.scopes.contains(ScopeType::Loop) { ERRO("Break keyword usada fora de um laço de repetição"); }
            SemanticNodeData::Terminal { value } => {
              if value.token_type == TokenType::KwBreak {
                if !scopes.contains(ScopeType::Loop) {
                  return Err(format!("Erro semântico: Comando 'break' fora de um laço de repetição na linha {} coluna {}", value.line, value.column).into());
                }
              }
            },
            _ => {},
          }
          return commandstat.semantic_analysis(scopes);
        }
        // Statement -> ;
        Ok(None)
      },
      SemanticNodeData::Term { unaryexpression, unaryexpression2, .. } => {
        let ReturnSem::Dado { tipo: tipo1, pos: exp1_pos } = unaryexpression.semantic_analysis(scopes)?.unwrap() else { panic!(); };
        if let Some(unaryexpression2) = unaryexpression2 {
          let ReturnSem::Dado { tipo: tipo2, pos: exp2_pos } = unaryexpression2.semantic_analysis(scopes)?.unwrap() else { panic!(); };
          let exp2_pos = exp2_pos.unwrap_or((0, 0));

          if tipo1 != tipo2 {
            return Err(format!("Erro semântico: tipos incompatíveis na expressão numérica na linha {} coluna {}", exp2_pos.0, exp2_pos.1).into());
          }
        }
        return Ok(Some(ReturnSem::Dado { tipo: tipo1, pos: exp1_pos }));
      },
      SemanticNodeData::Unaryexpression { factor, .. } => {
        factor.semantic_analysis(scopes)
      },
      SemanticNodeData::Vardecl {var_type, id, const_index} => {
        // Declared variable type
        let SemanticNodeData::Terminal { value: var_type_node } = var_type.children else { panic!() };
        let var_type = var_type_node.value.unwrap().get_keyword_type();

        // Declared variable name
        let SemanticNodeData::Terminal { value: id_node } = id.children else { panic!() };
        let ConstType::String(id_name) = id_node.value.clone().unwrap() else { panic!() };

        // Declared variable dimensions
        let mut token_index = vec![];
        if let Some(const_index) = const_index {
          let SemanticNodeData::ConstIndex { index } = const_index.children else { panic!() };
          for child in index.iter() {
            let SemanticNodeData::Terminal { value: token } = &child.children else { panic!(); };
            if let Some(ConstType::Int(index_value)) = &token.value { token_index.push(*index_value as u32); } 
            else { panic!();}
          }
        }

        // Insert the variable into the current scope
        let entry = SymbolEntry {
          appearances: vec![(id_node.line, id_node.column)],
          var_type: vec![var_type.clone()],
          const_index: token_index,
        };
        scopes.insert_symbol(id_name, entry)?;

        Ok(None)
      },
      SemanticNodeData::VarIndex {index} => {
        for i in index.iter() {
          i.semantic_analysis(scopes)?;
        }
        Ok(None)
      }, 
      SemanticNodeData::Terminal { value: token } => {
        match token.token_type {
          TokenType::Eof => {
            // # Pop global scope
            // EOF
            // EOF.scopes.pop()
            if scopes.stack.len() == 1 {
              scopes.pop_scope();
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
                Ok(Some(ReturnSem::Dado{ tipo: VarType::Int, pos: Some((token.line, token.column)) }))
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
                Ok(Some(ReturnSem::Dado{ tipo: VarType::Float, pos: Some((token.line, token.column)) }))
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
                Ok(Some(ReturnSem::Dado{ tipo: VarType::String, pos: Some((token.line, token.column)) }))
              } else {
                Err("Expected string constant".into())
              }
            } else {
              Err("Expected value for const_string".into())
            }
          },
          TokenType::Id => {
            // # LVALUE -> id VAR_INDEX
            // #  LVALUE.tipo = LVALUE.scopes.get(id)
            let ConstType::String(id_name) = token.value.clone().unwrap() else { panic!(); };
            // Count the appearance of the variable
            scopes.count_appearance(&id_name, token.line, token.column)?;
            let Some(symbol_entry) = scopes.get_symbol(&id_name) else {
              return Err(format!("Erro semântico: variável '{}' não declarada no escopo atual na linha {} columna {}", id_name, token.line, token.column).into());
            };
            let tipo = symbol_entry.var_type[0].clone();
            Ok(Some(ReturnSem::Dado { tipo, pos: Some((token.line, token.column)) }))
          },
          TokenType::VarType => {
            Ok(Some(ReturnSem::Dado{ tipo: token.get_type(), pos: Some((token.line, token.column)) }))
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

  /// generate TAC (Three Address Code) for the semantic node
  fn generate_code(&self, inh: &mut CodeAttrs) -> String {
    match &self.children {
      SemanticNodeData::Allocexpression { var_type, dimensions } => {
        // ALLOCEXPRESSION -> kw_new vartype VAR_INDEX
        // will not call generate_code for dimensions,
        // because it would print the dimensions between brackets
        let SemanticNodeData::VarIndex { index 
        } = &dimensions.children else { panic!(); };
        let tmp_dim = inh.create_temp();
        inh.code.push_str(&format!("{} = 1\n", tmp_dim));
        if !index.is_empty() {
          for child in index.iter() {
            let tmp0 = child.generate_code(inh);
            inh.code.push_str(&format!("{} = {} * {}\n", tmp_dim, tmp_dim, tmp0));
          }
        }
        // Generate code for the variable type
        let var_size = var_type.generate_code(inh);
        inh.code.push_str(&format!("param {}\n", var_size));
        let tmp = inh.create_temp();
        inh.code.push_str(&format!("param {}\n", tmp_dim));
        inh.code.push_str(&format!("{} = call alloc, 2", tmp));
        inh.code.push_str("\n");
        tmp
      },
      SemanticNodeData::Atribstat { lvalue, value } => {
        // ATRIBSTAT -> LVALUE op_assign ATRIBSTATEVALUE
        let tmp_lv = lvalue.generate_code(inh);
        let tmp_v = value.generate_code(inh);
        inh.code.push_str(&format!("{} = {}\n", tmp_lv, tmp_v));
        tmp_lv
      },
      SemanticNodeData::Atribstatevalue { expression, allocexpression, funccall } => {
        // ATRIBSTATEVALUE -> EXPRESSION 
        // ATRIBSTATEVALUE -> ALLOCEXPRESSION
        // ATRIBSTATEVALUE -> FUNCCALL
        if let Some(expression) = expression {
          expression.generate_code(inh)
        } else if let Some(allocexpression) = allocexpression {
          allocexpression.generate_code(inh)
        } else if let Some(funccall) = funccall {
          funccall.generate_code(inh)          
        } else {
          panic!("ATRIBSTATEVALUE is empty");
        }
      },
      SemanticNodeData::Constant { value, .. } => {
        // creates a new temporary variable for the constant
        let tmp = inh.create_temp();
        inh.code.push_str(&format!("{tmp} = {}\n", value.to_string()));
        tmp
      },
      SemanticNodeData::ConstIndex { index } => {
        // CONSTINDEX -> [CONSTANT1, CONSTANT2, CONSTANT3...]
        // CONST_INDEX -> [lbracket const_int rbracket]+
        for child in index.iter() {
          inh.code.push_str("[");
          child.generate_code(inh);
          inh.code.push_str("]");
        }
        String::new() // No temporary variable is created for CONSTINDEX
      },
      SemanticNodeData::Elsestat { statement } => {
        // ELSESTAT -> kw_else IFSTAT
        // ELSESTAT -> kw_else lbrace STATELIST rbrace
        statement.generate_code(inh)
      },
      SemanticNodeData::Expression { numexpression, numexpression2, op_expression } => {
        // EXPRESSION -> NUMEXPRESSION
        // EXPRESSION -> NUMEXPRESSION op_expression numexpression2
        // inh.code.push_str(&format!("{{\n"));

        let tmp = numexpression.generate_code(inh);
        if let Some(numexpression2) = numexpression2 {
          if let Some(op_expression) = op_expression {
            let tmp2 = numexpression2.generate_code(inh);
            inh.code.push_str(&format!("{} = {} ", tmp, tmp));
            op_expression.generate_code(inh);
            inh.code.push_str(&format!(" {}\n", tmp2));
          }
        }
        tmp
      },
      SemanticNodeData::Factor { expression, lvalue, constant } => {
        // FACTOR -> EXPRESSION
        // FACTOR -> LVALUE
        // FACTOR -> CONSTANT
        if let Some(expression) = expression {
          expression.generate_code(inh)
        } else if let Some(lvalue) = lvalue {
          let id = lvalue.generate_code(inh);
          let tmp = inh.create_temp();
          inh.code.push_str(&format!("{} = {}\n", tmp, id));
          tmp
        } else if let Some(constant) = constant {
          constant.generate_code(inh)
        } else {
            panic!("FACTOR is empty");
        }
      },
      SemanticNodeData::Forstat { init, condition, increment, body } => {
        // FORSTAT -> kw_for lparenthesis ATRIBSTAT semicolon EXPRESSION semicolon ATRIBSTAT rparenthesis lbrace STATELIST rbrace
        // creates a loop label
        let loop_start_label = inh.create_label();
        // generates code for the loop initialization
        init.generate_code(inh);
        // adds the loop start label to the code
        inh.code.push_str(&format!("{}:\n", loop_start_label));
        // generates code for the loop condition and
        // gets the previous return value, which is the result of the condition
        let cond_tmp = condition.generate_code(inh);
        // creates a label for the end of the loop
        let loop_end_label = inh.create_label();
        // inverts the condition and adds a goto to the end of the loop
        inh.code.push_str(&format!("if {} == 0 goto {}\n", cond_tmp, loop_end_label));
        // sets the end label as the break label
        inh.set_scope_end(loop_end_label.clone());
        // generates code for the loop body
        body.generate_code(inh);
        // generates code for the loop increment
        increment.generate_code(inh);
        // adds a goto to the start of the loop
        inh.code.push_str(&format!("goto {}\n", loop_start_label));
        // adds a label for the end of the loop
        inh.code.push_str(&format!("{}:\n", loop_end_label));
        cond_tmp
      },
      SemanticNodeData::Funccall { id, paramlistcall } => {
        // FUNCCALL -> id
        // FUNCCALL -> id PARAMLISTCALL
        let n_params = if let Some(paramlistcall) = paramlistcall {
          paramlistcall.generate_code(inh);
          let SemanticNodeData::Paramlistcall { paramlist } = &paramlistcall.children else { panic!(); };
          paramlist.len()
        } else {
          0
        };
        let tmp = inh.create_temp();
        inh.code.push_str(&format!("{} = call ", tmp));
        id.generate_code(inh);
        inh.code.push_str(&format!(", {}\n", n_params));
        tmp
      },
      SemanticNodeData::Funcdef { func_id, paramlist, statelist } => {
        // FUNCDEF -> func_id PARAMLIST STATELIST
        // FUNCDEF -> func_id STATELIST
        func_id.generate_code(inh);
        inh.code.push_str(":\n");
        if let Some(paramlist) = paramlist {
          paramlist.generate_code(inh);
        }
        statelist.generate_code(inh)
      },
      SemanticNodeData::Funclist { funclist } => {
        // FUNCLIST -> FUNCDEF+
        for func in funclist.iter() {
          func.generate_code(inh);
        }
        String::new() // No temporary variable is created for FUNCLIST
      },
      SemanticNodeData::Ifstat { condition, then_branch, else_branch } => {
        // IFSTAT -> kw_if lparenthesis EXPRESSION rparenthesis lbrace STATELIST rbrace ELSESTAT
        // calculates the condition, and
        // gets the previous return value, which is the result of the condition
        let cond_tmp = condition.generate_code(inh);
        // creates a label for the end of then branch
        let then_end_label = inh.create_label();
        // inverts the condition and adds a goto to the end of then branch
        inh.code.push_str(&format!("if {} == 0 goto {}\n", cond_tmp, then_end_label));
        // generates the code for the then branch
        then_branch.generate_code(inh);
        // adds a label for the end of the then branch
        inh.code.push_str(&format!("{}:\n", then_end_label));
        // generates the code for the else branch, if it exists
        if let Some(else_branch) = else_branch {
          // creates a label for the end of else branch
          let else_end_label = inh.create_label();
          // inverts the condition and adds a goto to the end of else branch
          inh.code.push_str(&format!("if {} goto {}\n", cond_tmp, else_end_label));
          // generates the code for the else branch
          else_branch.generate_code(inh);
          // adds a label for the end of the else branch
          inh.code.push_str(&format!("{}:\n", else_end_label));
        }
        cond_tmp // returns the temporary variable generated for the condition
      },
      SemanticNodeData::Lvalue { id, var_index } => {
        // LVALUE -> id
        // LVALUE -> id VARINDEX
        // let tmp = inh.create_temp();
        // inh.code.push_str(&format!("{} = ", tmp));
        let mut lval = if let SemanticNodeData::Terminal { value } = &id.children {
            format!("{}", value.value.as_ref().unwrap().to_string())
        } else {
          panic!("Expected Terminal for LVALUE id");
        };
        if let Some(var_index) = var_index {
          lval.push_str(var_index.generate_code(inh).as_str());
        }
        // inh.code.push_str("\n");
        lval
      },
      SemanticNodeData::Numexpression { term, op_numexpression, term2 } => {
        // NUMEXPRESSION -> term
        // NUMEXPRESSION -> term op_numexpressin term2
        // term may also be a Numexpression
        // inh.code.push_str(&format!("[\n"));
        // there should always be a previous return value
        let tmp = term.generate_code(inh);
        if let Some(term2) = term2 {
          let tmp2 = term2.generate_code(inh);
          if let Some(op_numexpression) = op_numexpression {
            inh.code.push_str(&format!("{} = {} ", tmp, tmp));
            op_numexpression.generate_code(inh);
            inh.code.push_str(&format!(" {}\n", tmp2));
          }
        } 
        // inh.code.push_str("]\n");
        tmp
      },
      SemanticNodeData::OpExpression { op } => {
        // OP_EXPRESSION -> op_eq
        // OP_EXPRESSION -> op_ne
        // OP_EXPRESSION -> op_ge
        // OP_EXPRESSION -> op_gt
        // OP_EXPRESSION -> op_le
        // OP_EXPRESSION -> op_lt
        let val = format!("{}", op.get_operator_type());
        inh.code.push_str(&val);
        val
      },
      SemanticNodeData::OpNumexpression { op } => {
        // OP_NUMEXPRESSION -> op_plus
        // OP_NUMEXPRESSION -> op_minus
        let val = format!("{}", op.get_operator_type());
        inh.code.push_str(&val);
        val
      },
      SemanticNodeData::OpTerm { op } => {
        // OP_TERM -> op_multiply
        // OP_TERM -> op_division
        // OP_TERM -> op_modular
        let val = format!("{}", op.get_operator_type());
        inh.code.push_str(&val);
        val
      },
      SemanticNodeData::Paramlist { .. } => {        
        // PARAMLIST -> (vartype id)+
        String::new()
      },
      SemanticNodeData::Paramlistcall { paramlist } => {
        // PARAMLISTCALL -> (id)+
        for param in paramlist.iter() {
          inh.code.push_str("param ");
          param.generate_code(inh);
          inh.code.push_str("\n");
        }
        String::new() // No temporary variable is created for PARAMLISTCALL
      },
      SemanticNodeData::Printstat { expression } => {
        // PRINTSTAT -> EXPRESSION
        // converts to TAC version of a print statement
        // gets the value of the expression
        let tmp = expression.generate_code(inh);
        // loads as a parameter to the print function
        inh.code.push_str(&format!("param {}\n", tmp));
        // calls the print function
        inh.code.push_str("call print, 1\n");
        tmp // returns the temporary variable used for the expression
      },
      SemanticNodeData::Program { funclist, statement } => {
        // PROGRAM -> FUNCLIST
        // PROGRAM -> STATEMENT
        if let Some(funclist) = funclist {
          funclist.generate_code(inh)
        } else if let Some(statement) = statement {
          statement.generate_code(inh)
        } else {
          panic!("PROGRAM is empty");
        }
      },
      SemanticNodeData::Readstat { lvalue } => {
        // READSTAT -> LVALUE
        // converts to TAC version of a read statement
        let tmp = lvalue.generate_code(inh);
        // call read function for the temporary variable
        inh.code.push_str(&format!("{} = call read, 0\n", tmp));
        tmp
      },
      SemanticNodeData::Returnstat { .. } => {
        // RETURNSTAT -> kw_return semicolon
        inh.code.push_str("return\n");
        String::new() // No temporary variable is created for RETURNSTAT
      },
      SemanticNodeData::Statelist { statelist } => {
        // STATELIST -> STATEMENT+
        let mut tmp = String::new();
        for statement in statelist.iter() {
          tmp = statement.generate_code(inh);
        }
        // returns the last temporary variable generated
        tmp
      },
      SemanticNodeData::Statement { vardecl, atribstat, ifstat, forstat, statelist, commandstat } => {
        // STATEMENT -> VARDECL 
        // STATEMENT -> ATRIBSTAT
        // STATEMENT -> IFSTAT
        // STATEMENT -> FORSTAT
        // STATEMENT -> STATELIST
        // os comandos são:
        // STATEMENT -> PRINTSTAT
        // STATEMENT -> READSTAT
        // STATEMENT -> RETURNSTAT
        // STATEMENT -> kw_break
        if let Some(vardecl) = vardecl {
          vardecl.generate_code(inh)
        } else if let Some(atribstat) = atribstat {
          atribstat.generate_code(inh)
        } else if let Some(ifstat) = ifstat {
          ifstat.generate_code(inh)
        } else if let Some(forstat) = forstat {
          forstat.generate_code(inh)
        } else if let Some(statelist) = statelist {
          statelist.generate_code(inh)
        } else if let Some(commandstat) = commandstat {
          commandstat.generate_code(inh)
        } else {
          String::new()
        }
      },
      SemanticNodeData::Term { unaryexpression, op_term, unaryexpression2 } => {
        // TERM -> UNARYEXPRESSION
        // TERM -> UNARYEXPRESSION op_term UNARYEXPRESSION
        // inh.code.push_str(&format!("(\n"));
        let tmp = unaryexpression.generate_code(inh);
        if let Some(unaryexpression2) = unaryexpression2 {
          let tmp2 = unaryexpression2.generate_code(inh);
          if let Some(op_term) = op_term {
            inh.code.push_str(&format!("{} = {} ", tmp, tmp));
            op_term.generate_code(inh);
            inh.code.push_str(&format!(" {}\n", tmp2));
          }
        };
        // inh.code.push_str(")\n");
        tmp
      },
      SemanticNodeData::Unaryexpression { op,  factor} => {
        // UNARYEXPRESSION -> FACTOR
        // UNARYEXPRESSION -> op FACTOR
        // inh.code.push_str(&format!("<\n --> {:?}\n", self));
        let tmp = factor.generate_code(inh);
        if let Some(op) = op {
          // If there is an operator, we need to create a temporary variable
          let tmp2 = inh.create_temp();
          inh.code.push_str(&format!("{} = ", tmp2));
          op.generate_code(inh);
          inh.code.push_str(&format!(" {}", tmp));
          tmp2
        } else {
          // If there is no operator, we just return the factor
          tmp
        }
      },
      SemanticNodeData::Vardecl { .. } => {
        //VARDECL -> var_type id
        //VARDECL -> var_type id CONSTINDEX
        // Vardecl is not used in code generation, 
        String::new()
      },
      SemanticNodeData::VarIndex { index } => {
        // VARINDEX -> [NUMEXPRESSION1, NUMEXPRESSION2, NUMEXPRESSION3]
        let mut output = String::new();
        for child in index.iter() {
          output.push_str("[\n");
          let tmp = child.generate_code(inh);
          output.push_str(&format!("{tmp}]\n"));
        }
        output
      },
      SemanticNodeData::Terminal { value: token } => {
        match token.token_type {
          TokenType::Id | TokenType::FuncId => {
            let t = format!("{}", token.value.as_ref().unwrap().to_string());
            inh.code.push_str(&t);
            t
          },
          TokenType::ConstInt | TokenType::ConstFloat | TokenType::ConstString => {
            let val =format!("{}\n", token.value.as_ref().unwrap().to_string());
            inh.code.push_str(&val);
            val
          },
          TokenType::ConstNull => {
            inh.code.push_str("0\n");
            String::from("0")
          },
          TokenType::VarType => {
            format!("{:?}", token.get_type())
          },
          TokenType::KwIf => {
            panic!("If keyword should not appear on generated code");
          },
          TokenType::KwFor => {
            panic!("For keyword should not appear on generated code");
          }
          TokenType::OpEq | TokenType::OpNe | TokenType::OpGt | TokenType::OpGe |
            TokenType::OpLt | TokenType::OpLe | TokenType::OpPlus | TokenType::OpMinus |
            TokenType::OpMultiply | TokenType::OpDivision | TokenType::OpModular
            => {
            let val = format!("{} ", token.token_type.get_operator_type());
            inh.code.push_str(&val);
            val
          },
          TokenType::KwBreak => {
            let br = inh.get_scope_label();
            inh.code.push_str(&format!("goto {}\n", br));
            String::new()
          },
          TokenType::KwNew => {
            // This is usually handled in the AllocExpression node
            panic!("New keyword should not appear on generated code");
          },
          TokenType::KwPrint => {
            // This is usually handled in the PrintStat node
            panic!("Print keyword should not appear on generated code");
          },
          TokenType::KwRead => {
            // This is usually handled in the ReadStat node
            panic!("Read keyword should not appear on generated code");
          },
          _ => {
            panic!("Unexpected token type in code generation: {:?}", token.token_type);
          }
            
        }
      }
    }
  }

  fn save(&self, output: &mut String, count: &mut u32) {
    match &self.children {
      SemanticNodeData::Allocexpression { var_type, dimensions } => {        
        let name = format!("{}", count);
        output.push_str(&format!("  {} [label=\"AllocExpression\"]\n", name,));
        output.push_str(&format!("  {} -> {}\n", name, *count+1));
        *count += 1;
        var_type.save(output, count);
        output.push_str(&format!("  {} -> {}\n", name, *count+1));
        *count += 1;
        dimensions.save(output, count);
      },
      SemanticNodeData::Atribstat { lvalue, value } => {
        let name = format!("{}", count);
        output.push_str(&format!("  {} [label=\"AtribStatement\"]\n", count));
        output.push_str(&format!("  {} -> {}\n", name, *count+1));
        *count += 1;
        lvalue.save(output, count);
        output.push_str(&format!("  {} -> {}\n", name, *count+1));
        *count += 1;
        value.save(output, count);
      },
      SemanticNodeData::Atribstatevalue { expression, allocexpression, funccall } => {
        let name = format!("{}", count);
        output.push_str(&format!("  {} [label=\"AtribStatementValue\"]\n", count));
        if let Some(expression) = expression {
          output.push_str(&format!("  {} -> {}\n", name, *count+1));
          *count += 1;
          expression.save(output, count);
        }
        if let Some(allocexpression) = allocexpression {
          output.push_str(&format!("  {} -> {}\n", name, *count+1));
          *count += 1;
          allocexpression.save(output, count);
        }
        if let Some(funccall) = funccall {
          output.push_str(&format!("  {} -> {}\n", name, *count+1));
          *count += 1;
          funccall.save(output, count);
        }
      },
      SemanticNodeData::Constant { value, .. } => {
        output.push_str(&format!("  {} [label=\"{:?}\"]\n", count, value));
      },
      SemanticNodeData::ConstIndex { index } => {
        let name = format!("{}", count);
        output.push_str(&format!("  {} [label=\"ConstIndex\"]\n", count));
        for i in index.iter() {
          output.push_str(&format!("  {} -> {}\n", name, *count+1));
          *count += 1;
          i.save(output, count);
        }
      },
      SemanticNodeData::Elsestat { statement } => {
        let name = format!("{}", count);
        output.push_str(&format!("  {} [label=\"ElseStatement\"]\n", count));
        output.push_str(&format!("  {} -> {}\n", name, *count+1));
        *count += 1;
        statement.save(output, count);
      },
      SemanticNodeData::Expression { numexpression, numexpression2, op_expression } => {
        let name = format!("{}", count);
        output.push_str(&format!("  {} [label=\"Expression\"]\n", count,));
        output.push_str(&format!("  {} -> {}\n", name, *count+1));
        *count += 1;
        numexpression.save(output, count);
        if let Some(op_expression) = op_expression {
          output.push_str(&format!("  {} -> {}\n", name, *count+1));
          *count += 1;
          op_expression.save(output, count);
        }
        if let Some(numexpression2) = numexpression2 {
          output.push_str(&format!("  {} -> {}\n", name, *count+1));
          *count += 1;
          numexpression2.save(output, count);
        }
      },
      SemanticNodeData::Factor { expression, lvalue, constant } => {
        let name = format!("{}", count);
        output.push_str(&format!("  {} [label=\"Factor\"]\n", count));
        if let Some(expression) = expression {
          output.push_str(&format!("  {} -> {}\n", name, *count+1));
          *count += 1;
          expression.save(output, count);
        }
        if let Some(lvalue) = lvalue {
          output.push_str(&format!("  {} -> {}\n", name, *count+1));
          *count += 1;
          lvalue.save(output, count);
        }
        if let Some(constant) = constant {
          output.push_str(&format!("  {} -> {}\n", name, *count+1));
          *count += 1;
          constant.save(output, count);
        }
      },
      SemanticNodeData::Forstat { init, condition, increment, body } => {
        let name = format!("{}", count);
        output.push_str(&format!("  {} [label=\"ForStatement\"]\n", count));
        output.push_str(&format!("  {} -> {}\n", name, *count+1));
        *count += 1;
        init.save(output, count);
        output.push_str(&format!("  {} -> {}\n", name, *count+1));
        *count += 1;
        condition.save(output, count);
        output.push_str(&format!("  {} -> {}\n", name, *count+1));
        *count += 1;
        increment.save(output, count);
        output.push_str(&format!("  {} -> {}\n", name, *count+1));
        *count += 1;
        body.save(output, count);
      },
      SemanticNodeData::Funccall { id, paramlistcall } => {
        let name = format!("{}", count);
        output.push_str(&format!("  {} [label=\"FuncCall\"]\n", count));
        output.push_str(&format!("  {} -> {}\n", name, *count+1));
        *count += 1;
        id.save(output, count);
        if let Some(paramlistcall) = paramlistcall {
          output.push_str(&format!("  {} -> {}\n", name, *count+1));
          *count += 1;
          paramlistcall.save(output, count);
        }
      },
      SemanticNodeData::Funcdef { func_id, paramlist, statelist } => {
        let name = format!("{}", count);
        output.push_str(&format!("  {} [label=\"FuncDef\"]\n", count));
        output.push_str(&format!("  {} -> {}\n", name, *count+1));
        *count += 1;
        func_id.save(output, count);
        if let Some(paramlist) = paramlist {
          output.push_str(&format!("  {} -> {}\n", name, *count+1));
          *count += 1;
          paramlist.save(output, count);
        }
        output.push_str(&format!("  {} -> {}\n", name, *count+1));
        *count += 1;
        statelist.save(output, count);
      },
      SemanticNodeData::Funclist { funclist } => {
        let name = format!("{}", count);
        output.push_str(&format!("  {} [label=\"FuncList\"]\n", count));
        for func in funclist.iter() {
          output.push_str(&format!("  {} -> {}\n", name, *count+1));
          *count += 1;
          func.save(output, count);
        }
      },
      SemanticNodeData::Ifstat { condition, then_branch, else_branch } => {
        let name = format!("{}", count);
        output.push_str(&format!("  {} [label=\"IfStatement\"]\n", name));
        output.push_str(&format!("  {} -> {}\n", name, *count+1));
        *count += 1;
        condition.save(output, count);
        output.push_str(&format!("  {} -> {}\n", name, *count+1));
        *count += 1;
        then_branch.save(output, count);
        if let Some(else_branch) = else_branch {
          output.push_str(&format!("  {} -> {}\n", name, *count+1));
          *count += 1;
          else_branch.save(output, count);
        }
      },
      SemanticNodeData::Lvalue { id, var_index } => {
        let name = format!("{}", count);
        output.push_str(&format!("  {} [label=\"LValue\"]\n", count));
        output.push_str(&format!("  {} -> {}\n", name, *count+1));
        *count += 1;
        id.save(output, count);
        if let Some(var_index) = var_index {
          output.push_str(&format!("  {} -> {}\n", name, *count+1));
          *count += 1;
          var_index.save(output, count);
        }
      },
      SemanticNodeData::Numexpression { term, op_numexpression, term2 } => {
        let name = format!("{}", count);
        output.push_str(&format!("  {} [label=\"Numexpression\"]\n", count));
        output.push_str(&format!("  {} -> {}\n", name, *count+1));
        *count += 1;
        term.save(output, count);

        if let Some(op_numexpression) = op_numexpression {
          output.push_str(&format!("  {} -> {}\n", name, *count+1));
          *count += 1;
          op_numexpression.save(output, count);
        }
        if let Some(term2) = term2 {
          output.push_str(&format!("  {} -> {}\n", name, *count+1));
          *count += 1;
          term2.save(output, count);
        }
      },
      SemanticNodeData::OpExpression { op } => {
        output.push_str(&format!("  {} [label=\"{:?}\"]\n", count, op));
      },
      SemanticNodeData::OpNumexpression { op } => {
        output.push_str(&format!("  {} [label=\"{:?}\"]\n", count, op));
      },
      SemanticNodeData::OpTerm { op } => {
        output.push_str(&format!("  {} [label=\"{:?}\"]\n", count, op));
      },
      SemanticNodeData::Paramlist { paramlist } => {
        let name = format!("{}", count);
        output.push_str(&format!("  {} [label=\"ParamList\"]\n", count));
        for param in paramlist.iter() {
          output.push_str(&format!("  {} -> {}\n", name, *count+1));
          *count += 1;
          param.save(output, count);
        }
      },
      SemanticNodeData::Paramlistcall { paramlist } => {
        let name = format!("{}", count);
        output.push_str(&format!("  {} [label=\"ParamListCall\"]\n", count));
        for param in paramlist.iter() {
          output.push_str(&format!("  {} -> {}\n", name, *count+1));
          *count += 1;
          param.save(output, count);
        }
      },
      SemanticNodeData::Printstat { expression } => {
        let name = format!("{}", count);
        output.push_str(&format!("  {} [label=\"PrintStatement\"]\n", count));
        output.push_str(&format!("  {} -> {}\n", name, *count+1));
        *count += 1;
        expression.save(output, count);
      },
      SemanticNodeData::Program { funclist, statement } => {
        let name = format!("{}", count);
        output.push_str(&format!("  {} [label=\"Program\"]\n", count));
        if let Some(funclist) = funclist {
          output.push_str(&format!("  {} -> {}\n", name, *count+1));
          *count += 1;
          funclist.save(output, count);
        }
        if let Some(statement) = statement {
          output.push_str(&format!("  {} -> {}\n", name, *count+1));
          *count += 1;
          statement.save(output, count);
        }
      },
      SemanticNodeData::Readstat { lvalue } => {
        let name = format!("{}", count);
        output.push_str(&format!("  {} [label=\"ReadStatement\"]\n", name));
        output.push_str(&format!("  {} -> {}\n", name, *count+1));
        *count += 1;
        lvalue.save(output, count);
      },
      SemanticNodeData::Returnstat { .. } => {
        output.push_str(&format!("  {} [label=\"ReturnStatement\"]\n", count));
      },
      SemanticNodeData::Statelist { statelist } => {
        let name = format!("{}", count);
        output.push_str(&format!("  {} [label=\"StateList\"]\n", count));
        for statement in statelist.iter() {
          output.push_str(&format!("  {} -> {}\n", name, *count+1));
          *count += 1;
          statement.save(output, count);
        }
      },
      SemanticNodeData::Statement { vardecl, atribstat, ifstat, forstat, statelist, commandstat } => {
        let name = format!("{}", count);
        output.push_str(&format!("  {} [label=\"Statement\"]\n", count));
        if let Some(vardecl) = vardecl {
          output.push_str(&format!("  {} -> {}\n", name, *count+1));
          *count += 1;
          vardecl.save(output, count);
        }
        if let Some(atribstat) = atribstat {
          output.push_str(&format!("  {} -> {}\n", name, *count+1));
          *count += 1;
          atribstat.save(output, count);
        }
        if let Some(ifstat) = ifstat {
          output.push_str(&format!("  {} -> {}\n", name, *count+1));
          *count += 1;
          ifstat.save(output, count);
        }
        if let Some(forstat) = forstat {
          output.push_str(&format!("  {} -> {}\n", name, *count+1));
          *count += 1;
          forstat.save(output, count);
        }
        if let Some(statelist) = statelist {
          output.push_str(&format!("  {} -> {}\n", name, *count+1));
          *count += 1;
          statelist.save(output, count);
        }
        if let Some(commandstat) = commandstat {
          output.push_str(&format!("  {} -> {}\n", name, *count+1));
          *count += 1;
          commandstat.save(output, count);
        }
      },
      SemanticNodeData::Term { unaryexpression, op_term, unaryexpression2 } => {
        let name = format!("{}", count);
        output.push_str(&format!("  {} [label=\"Term\"]\n", count));
        output.push_str(&format!("  {} -> {}\n", name, *count+1));
        *count += 1;
        unaryexpression.save(output, count);
        if let Some(op_term) = op_term {
          output.push_str(&format!("  {} -> {}\n", name, *count+1));
          *count += 1;
          op_term.save(output, count);
        }
        if let Some(unaryexpression2) = unaryexpression2 {
          output.push_str(&format!("  {} -> {}\n", name, *count+1));
          *count += 1;
          unaryexpression2.save(output, count);
        }
      },
      SemanticNodeData::Unaryexpression { op, factor } => {
        let name = format!("{}", count);
        output.push_str(&format!("  {} [label=\"Unaryexpression\"]\n", count));
        if let Some(op) = op {
          output.push_str(&format!("  {} -> {}\n", name, *count+1));
          *count += 1;
          op.save(output, count);
        }
        output.push_str(&format!("  {} -> {}\n", name, *count+1));
        *count += 1;
        factor.save(output, count);
      },
      SemanticNodeData::Vardecl { var_type, id, const_index } => {
        let name = format!("{}", count);
        output.push_str(&format!("  {} [label=\"Vardecl\"]\n", count));
        output.push_str(&format!("  {} -> {}\n", name, *count+1));
        *count += 1;
        var_type.save(output, count);
        output.push_str(&format!("  {} -> {}\n", name, *count+1));
        *count += 1;
        id.save(output, count);
        if let Some(const_index) = const_index {
          output.push_str(&format!("  {} -> {}\n", name, *count+1));
          *count += 1;
          const_index.save(output, count);
        }
      },
      SemanticNodeData::VarIndex { index } => {
        let name = format!("{}", count);
        output.push_str(&format!("  {} [label=\"VarIndex\"]\n", count));
        for i in index.iter() {
          output.push_str(&format!("  {} -> {}\n", name, *count+1));
          *count += 1;
          i.save(output, count);
        }
      },
      SemanticNodeData::Terminal { value: token } => {
        if let Some(value) = &token.value {
          let nome = format!("{:?}", value).replace("\"", "\\\"");
          output.push_str(&format!("  {} [label=\"{}\"]\n", count, nome));
        }
      }
    }
  }

  fn create_expression_tree(&self, trees: &mut Vec<ExpressionTree>) -> Option<ExpressionTreeNode> {
    match &self.children {
      SemanticNodeData::Atribstat { value, .. } => {
        value.create_expression_tree(trees);
        None
      },
      SemanticNodeData::Atribstatevalue { expression, .. } => {
        // ATRIBSTATEVALUE -> EXPRESSION 
        if let Some(expression) = expression { expression.create_expression_tree(trees); None }
        else { None }
      },
      SemanticNodeData::Constant { value, .. } => {
        // CONSTANT -> const_int | const_float | const_string 
        // CONSTANT.ptr = const.ptr
        match value {
          ConstType::Int(i) => { Some(ExpressionTreeNode::Operand{ value: Operand::Integer(*i) }) },
          ConstType::Float(f) => { Some(ExpressionTreeNode::Operand{ value: Operand::Float(*f) })},
          ConstType::String(s) => { Some(ExpressionTreeNode::Operand{ value: Operand::String(s.clone()) }) },
        }
      },
      SemanticNodeData::Elsestat { statement } => {
        statement.create_expression_tree(trees);
        None
      },
      SemanticNodeData::Expression { numexpression, op_expression, numexpression2 } => {
        // Expression -> NumExpression | NumExpression OpExpression NumExpression
        // Nesse nodo, é criada a raiz da árvore de expressão, e a árvore é inserida no vetor de árvores para retorno da função
        // Em todos os outros nodos, são criados e retornados os outros nodos da árvore.

        let root = match op_expression {
          // EXPRESSION -> NUMEXPRESSION op_expression numexpression2 
          //  EXPRESSION_1.ptr = NODE(op_expression.op, NUMEXPRESSION.ptr, NUMEXPRESSION2.ptr)
          Some(op_expression) => {
            let n1 = numexpression.create_expression_tree(trees).unwrap();
            let n2 = numexpression2.clone().unwrap().create_expression_tree(trees).unwrap();
            let SemanticNodeData::OpExpression { op } = op_expression.children else { panic!(); }; 
            ExpressionTreeNode::BinaryOperator { 
              operator: op.get_operator_type(),
              left: Box::new(n1),
              right: Box::new(n2)
            }
          }
          // EXPRESSION -> NUMEXPRESSION 
          //   EXPRESSION.ptr -> NUMEXPRESSION.ptr 
          None => {
            numexpression.create_expression_tree(trees).unwrap()
          }
        };
        let tree = ExpressionTree { root };
        trees.push(tree);
        None
      },
      SemanticNodeData::Factor { expression, lvalue, constant } => {
        let node;
        // FACTOR -> EXPRESSION
        //  FACTOR.ptr = expression.ptr
        if let Some(expression) = expression {
          node = expression.create_expression_tree(trees);
        } 
        // FACTOR -> LVALUE
        //  FACTOR.ptr = lvalue.ptr
        else if let Some(lvalue) = lvalue {
          node = lvalue.create_expression_tree(trees);
        } 
        // FACTOR -> constant
        //  FACTOR.ptr = constant.ptr
        else if let Some(constant) = constant {
          node = constant.create_expression_tree(trees);
        } else {
          panic!();
        }
        node
      },
      SemanticNodeData::Forstat { init, condition, increment, body } => {
        init.create_expression_tree(trees);
        condition.create_expression_tree(trees);
        increment.create_expression_tree(trees);
        body.create_expression_tree(trees);
        None
      },
      SemanticNodeData::Funcdef { statelist, .. } => {
        statelist.create_expression_tree(trees);
        None
      },
      SemanticNodeData::Funclist { funclist } => {
        for func in funclist.iter() { func.create_expression_tree(trees); }
        None
      },
      SemanticNodeData::Ifstat { condition, then_branch, else_branch } => {
        condition.create_expression_tree(trees);
        then_branch.create_expression_tree(trees);
        if let Some(else_branch) = else_branch {
          else_branch.create_expression_tree(trees);
        }
        None
      },
      SemanticNodeData::Lvalue { id, var_index } => {
        // LVALUE -> id
        // LVALUE -> id VARINDEX
        //    LVALUE.ptr = id.ptr
        if let Some(var_index) = var_index { var_index.create_expression_tree(trees); }
        let SemanticNodeData::Terminal { value: id_node } = &id.children else { panic!(); };

        if let Some(ConstType::String(id_name)) = &id_node.value {
          Some(ExpressionTreeNode::Operand { value: Operand::Identifier(id_name.clone())})
        } else {
          panic!("Expected variable identifier in LValue");
        }
      },
      SemanticNodeData::Numexpression { term, op_numexpression, term2 } => {
        let root = match op_numexpression {
          // NUMEXPRESSION -> NUMEXPRESSION op_numexpression TERM
          //  NUMEXPRESSION_1.ptr = NODE(op_numexpression.op, NUMEXPRESSION_2.ptr, TERM.ptr)
          Some(op_numexpression) => {
            let n1 = term.create_expression_tree(trees).unwrap();
            let n2 = term2.clone().unwrap().create_expression_tree(trees).unwrap();
            let SemanticNodeData::OpNumexpression { op } = op_numexpression.children else { panic!(); };
            ExpressionTreeNode::BinaryOperator { 
              operator: op.get_operator_type(),
              left: Box::new(n1),
              right: Box::new(n2)
            }
          // NUMEXPRESSION -> TERM
          //  NUMEXPRESSION.ptr = TERM.ptr
          }
          // NUMERICEXPRESSION -> TERM
          //  NUMERICEXPRESSION.ptr = TERM.ptr
          None => {
            term.create_expression_tree(trees).unwrap()
          }
        };
        Some(root)
      },
      SemanticNodeData::Printstat { expression } => {
        expression.create_expression_tree(trees);
        None
      },
      SemanticNodeData::Program { funclist, statement } => {
        if let Some(funclist) = funclist { funclist.create_expression_tree(trees); }
        if let Some(statement) = statement { statement.create_expression_tree(trees); }
        None
      },
      SemanticNodeData::Statelist { statelist } => {
        for statement in statelist.iter() {
          statement.create_expression_tree(trees);
        }
        None
      },
      SemanticNodeData::Statement { atribstat, ifstat, forstat, statelist, commandstat, .. } => {
        if let Some(atribstat) = atribstat { atribstat.create_expression_tree(trees); }
        if let Some(ifstat) = ifstat { ifstat.create_expression_tree(trees); }
        if let Some(forstat) = forstat { forstat.create_expression_tree(trees); }
        if let Some(statelist) = statelist { statelist.create_expression_tree(trees); }
        if let Some(commandstat) = commandstat { commandstat.create_expression_tree(trees); }
        None
      },
      SemanticNodeData::Term { unaryexpression, op_term, unaryexpression2 } => {
        let root = match op_term {
          // TERM -> TERM_1 op_term UNARYEXPRESSION
          //  TERM.ptr = NODE(op_term.op, TERM_1.ptr, UNARYEXPRESSION.ptr)
          Some(op_term) => {
            let n1 = unaryexpression.create_expression_tree(trees).unwrap();
            let n2 = unaryexpression2.clone().unwrap().create_expression_tree(trees).unwrap();
            let SemanticNodeData::OpTerm { op } = op_term.children else { panic!(); };
            ExpressionTreeNode::BinaryOperator { 
              operator: op.get_operator_type(),
              left: Box::new(n1),
              right: Box::new(n2)
            }
          }
          // TERM -> UNARYEXPRESSION
          //  TERM.ptr = UNARYEXPRESSION.ptr
          None => { unaryexpression.create_expression_tree(trees).unwrap() }
        };
        Some(root)
      },
      SemanticNodeData::Terminal { value } => {
        // token.ptr = NODE(operando, token.value)
        match value.token_type {
          TokenType::ConstInt => {
            if let Some(ConstType::Int(i)) = &value.value {
              Some(ExpressionTreeNode::Operand { value: Operand::Integer(*i) })
            } else {
              panic!("Expected integer constant");
            }
          },
          TokenType::ConstFloat => {
            if let Some(ConstType::Float(f)) = &value.value {
              Some(ExpressionTreeNode::Operand { value: Operand::Float(*f) })
            } else {
              panic!("Expected float constant");
            }
          },
          TokenType::ConstString => {
            if let Some(ConstType::String(s)) = &value.value {
              Some(ExpressionTreeNode::Operand { value: Operand::String(s.clone()) })
            } else {
              panic!("Expected string constant");
            }
          },
          TokenType::Id => {
            if let Some(ConstType::String(id_name)) = &value.value {
              Some(ExpressionTreeNode::Operand { value: Operand::Identifier(id_name.clone()) })
            } else {
              panic!("Expected variable identifier");
            }
          },
          _ => None,
        }
      },
      SemanticNodeData::Unaryexpression { op, factor } => {
        // UNARYEXPRESSION -> op FACTOR
        //  UNARYEXPRESSION.ptr = NODE(op.op, FACTOR.ptr)
        match op {
          Some(op) => {
            let SemanticNodeData::OpNumexpression { op } = op.children else { panic!(); };
            Some(ExpressionTreeNode::UnaryOperator {
              operator: op.get_operator_type(),
              operand: Box::new(factor.create_expression_tree(trees).unwrap())
            })
          },
          // UNARYEXPRESSION -> FACTOR
          //  UNARYEXPRESSION.ptr -> FACTOR.ptr
          None => { factor.create_expression_tree(trees) }
        }
      },
      SemanticNodeData::VarIndex { index } => {
        // VARINDEX -> [NUMEXPRESSION1, NUMEXPRESSION2, NUMEXPRESSION3]
        for i in index.iter() { i.create_expression_tree(trees); }
        None
      },
      _ => { None }
    }
  }
}

pub struct SemanticTree {
  pub root: SemanticNode,
  pub scopes: ScopeStack
}

impl SemanticTree {
  pub fn semantic_analysis(&mut self) -> Result<(), Box<dyn Error>> {
    // Perform semantic analysis on the syntax tree
    // This is where we would check for variable declarations, types, etc.
    // For now, we will just print the structure of the semantic tree
    self.root.semantic_analysis(&mut self.scopes)?;
    Ok(())
  }

  pub fn generate_code(&self) -> String {
    let mut code_attrs = CodeAttrs::new();
    self.root.generate_code(&mut code_attrs);
    code_attrs.code
  }

  pub fn create_expression_trees(&self) -> Vec<ExpressionTree> {
    let mut trees = Vec::new();
    self.root.create_expression_tree(&mut trees);
    trees
  }

  pub fn output_stats(&self, output: &mut String) {
    output.push_str(&format!("Análise semântica concluída com sucesso.\n"));
    output.push_str(&format!("Árvore semântica construída a partir da árvore sintática:\n"));
    let mut ast = "Visualize a árvore colando este arquivo em https://dreampuf.github.io/GraphvizOnline/?engine=dot\ndigraph G {\n".to_string();
    self.root.save(&mut ast, &mut 0);
    output.push_str(&format!("{}}}\n", ast));
    let expression_trees = self.create_expression_trees();
    output.push_str(&format!("Árvores de expressão geradas: {} [\n", expression_trees.len()));
    for (i, tree) in expression_trees.iter().enumerate() {
      output.push_str(&format!("Árvore de expressão {}:\n{}\n", i + 1, tree.output()));
    }
    output.push_str(&format!("]\n"));
    output.push_str(&format!("Tipos de símbolos por escopo:\n{}\n", self.scopes.output));
    output.push_str(&format!("Verificação de tipos: Ok\n"));
    output.push_str(&format!("Verificação identificadores de escopos: Ok\n"));
    output.push_str(&format!("Verificação de comandos por escopo: Ok\n"));
  }
}