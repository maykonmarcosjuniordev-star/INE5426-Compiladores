use std::error::Error; 
use crate::semantic::SemanticTree;
use crate::token::Token;
use crate::grammar::token_type::TokenType;
use crate::grammar::non_terminals::NonTerminal;
use crate::semantic::SemanticNode;
use crate::grammar::semantic_node::SemanticNodeData;
use std::collections::HashMap;
use std::rc::Rc;
use std::io::Write;
use crate::scope_stack::ScopeStack;
#[derive(Clone)] 
pub enum Symbol {
  NonTerminal(NonTerminal),
  Terminal(TokenType, Option<Token>),
}
impl std::fmt::Debug for Symbol {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Symbol::NonTerminal(nt) => write!(f, "{:?}", nt),
      Symbol::Terminal(tt, _) => write!(f, "{:?}", tt),
    }
  }
}
pub type ParseTable = HashMap<(NonTerminal, TokenType), u32>;
#[derive(Clone)]
struct Node {
  value: Symbol,
  children: Vec<Box<Node>>,
  parse_table: Rc<ParseTable>,
  rules: Rc<Vec<(NonTerminal, Option<Vec<Symbol>>)>>,
  scopes: Rc<ScopeStack>,
}
impl Node {
  fn new(
    value: Symbol,
    parse_table: Rc<HashMap<(NonTerminal, TokenType), u32>>,
    rules: Rc<Vec<(NonTerminal, Option<Vec<Symbol>>)>>,
    scopes: Rc<ScopeStack>,
  ) -> Self {
    Node {
      value,
      children: vec![],
      parse_table,
      rules,
      scopes
    }
  }
  fn parse(&mut self, tokens: &Vec<Token>, index: &mut usize) -> Result<(), Box<dyn Error>> {
    let current_token = &tokens[*index];
    match &self.value {
      Symbol::Terminal(token_type, _) => {
        // Se o token lido for diferente do esperado, retorna um erro sintático
        if *token_type != current_token.token_type { 
          return Err(format!("Erro sintático: esperava {:?}, mas encontrou {:?} na linha {}, coluna {}", token_type, current_token.token_type, current_token.line, current_token.column).into());
        }
        // Caso contrário, avança para o próximo token
        self.value = Symbol::Terminal(*token_type, Some(current_token.clone()));
        *index += 1;
        Ok(())
      }
      Symbol::NonTerminal(non_terminal) => {
        // Se a tabela LL1 não contiver uma entrada para o não terminal e o token atual, retorna um erro sintático
        let Some(rule_index) = self.parse_table.get(&(*non_terminal, current_token.token_type)) else {
          return Err(format!("Erro sintático: não há regra para {:?} com o token {:?} na linha {}, coluna {}", non_terminal, current_token.token_type, current_token.line, current_token.column).into());
        };
        let rule_index = *rule_index;
        // Se a produção for vazia, não precisa fazer nada
        let Some(body) = &self.rules[rule_index as usize].1 else {
          return Ok(());
        };
        // Se a produção não for vazia, cria os nós da produção
        for symbol in body {
          let new_symbol = match symbol {
            Symbol::NonTerminal(nt) => Symbol::NonTerminal(nt.clone()),
            Symbol::Terminal(tt, _) => Symbol::Terminal(tt.clone(), None),
          };
          let mut child = Box::new(Node::new(new_symbol, Rc::clone(&self.parse_table), Rc::clone(&self.rules), Rc::clone(&self.scopes)));
          child.parse(tokens, index)?;
          self.children.push(child);
        }
        Ok(())
      }
    }
  }

  /// Regras semânticas para criação da AST,
  /// Nessa etapa, todos os outros nós serão apenas transformados em nós semânticos.
  /// Já para os nós relacionados a expressões, serão aplicadas as regras semânticas específicas para condensar a AST.
  fn visit(&self, inh: Option<&Vec<SemanticNode>>) -> SemanticNode {
    match &self.value {
      Symbol::Terminal(_, token) => {
        // Cria um nó semântico terminal com o tipo do token
        SemanticNode {
          children: SemanticNodeData::Terminal { value: token.clone().unwrap() },
        }
      },
      Symbol::NonTerminal(NonTerminal::Program) => {
        if self.children.len() != 1 { panic!() }
        let child = self.children[0].clone();
        match child.value {
          Symbol::NonTerminal(NonTerminal::Funclist) => {
            SemanticNode {
              children: SemanticNodeData::Program {
                funclist: Some(Box::new(child.visit(None))),
                statement: None,
              },
            }
          },
          Symbol::NonTerminal(NonTerminal::Statement) => {
            SemanticNode {
              children: SemanticNodeData::Program {
                funclist: None,
                statement: Some(Box::new(child.visit(None))),
              },
            }
          },
          _ => panic!()
        }
      },
      Symbol::NonTerminal(NonTerminal::Funclist) => {
        match self.children.len() {
          // FUNCLIST -> ''
          0 => {
            SemanticNode {
              children: SemanticNodeData::Funclist { funclist: inh.unwrap().clone() },
            }
          },
          // FUNCLIST -> FUNCDEF FUNCLIST
          2 => {
            let mut funclist = match inh {
              None => vec![],
              Some(inh) => inh.clone(),
            };
            funclist.push(self.children[0].visit(None));
            self.children[1].visit(Some(&funclist))
          },
          _ => panic!()
        }        
      }, 
      Symbol::NonTerminal(NonTerminal::Funcdef) => {
        if self.children.len() != 8 { panic!() }
        // FUNCDEF -> kw_def func_id lparenthesis PARAMLIST rparenthesis lbrace STATELIST rbrace
        // TODO: funcdef node must store function_id
        SemanticNode {
          children: SemanticNodeData::Funcdef {
            func_id: Box::new(self.children[1].visit(None)),
            paramlist: if self.children[3].children.len() > 0 {
              Some(Box::new(self.children[3].visit(None)))
            } else {
              None
            },
            statelist: Box::new(self.children[6].visit(None)),
          },
        }
      }, 
      Symbol::NonTerminal(NonTerminal::Paramlist) => {
        match self.children.len() {
          // PARAMLIST -> ''
          0 => {
            SemanticNode {
              children: SemanticNodeData::Paramlist { paramlist: vec![] },
            }
          },
          // PARAMLIST -> var_type id PARAMLIST1
          3 => {
            let inh = vec![self.children[0].visit(None), self.children[1].visit(None)];
            self.children[2].visit(Some(&inh))
          }
          _ => panic!()
        }
      }, 
      Symbol::NonTerminal(NonTerminal::Paramlist1) => {
        match self.children.len() {
          // PARAMLIST1 -> ''
          0 => {
            SemanticNode {
              children: SemanticNodeData::Paramlist { paramlist: inh.unwrap().clone() },
            }
          },
          // PARAMLIST1 -> comma var_type id PARAMLIST1
          4 => {
            let mut new_params = match inh {
              None => vec![],
              Some(inh) => inh.clone(),
            };
            new_params.push(self.children[1].visit(None));
            new_params.push(self.children[2].visit(None));
            self.children[2].visit(Some(&new_params))
          },
          _ => panic!()
        }
      }, 
      Symbol::NonTerminal(NonTerminal::Statelist) => {
        if self.children.len() != 2 { panic!() }
        let statelist = vec![self.children[0].visit(None)];
        self.children[1].visit(Some(&statelist))
      }, 
      Symbol::NonTerminal(NonTerminal::Statelist1) => {
        match self.children.len() {
          // STATELIST1 -> ''
          0 => {
            SemanticNode {
              children: SemanticNodeData::Statelist { statelist: inh.unwrap().clone() },
            }
          },
          // STATELIST1 -> STATEMENT STATELIST1
          2 => {
            let mut statelist = match inh {
              None => vec![],
              Some(inh) => inh.clone(),
            };
            statelist.push(self.children[0].visit(None));
            self.children[1].visit(Some(&statelist))
          },
          _ => panic!()
        }
      }, 
      Symbol::NonTerminal(NonTerminal::Statement) => {
        match self.children[0].value {
          // STATEMENT -> Vardecl semicolon
          Symbol::NonTerminal(NonTerminal::Vardecl) => {
            SemanticNode {
              children: SemanticNodeData::Statement {
                vardecl: Some(Box::new(self.children[0].visit(None))),
                atribstat: None,
                ifstat: None,
                forstat: None,
                statelist: None,
                commandstat: None,
              }
            }
          },
          // STATEMENT -> ATRIBSTAT semicolon
          Symbol::NonTerminal(NonTerminal::Atribstat) => {
            SemanticNode {
              children: SemanticNodeData::Statement {
                vardecl: None,
                atribstat: Some(Box::new(self.children[0].visit(None))),
                ifstat: None,
                forstat: None,
                statelist: None,
                commandstat: None,
              }
            }
          },
          // STATEMENT -> (PRINTSTAT | READSTAT | RETURNSTAT | kw_break) semicolon
          Symbol::NonTerminal(NonTerminal::Printstat) | Symbol::NonTerminal(NonTerminal::Readstat) | Symbol::NonTerminal(NonTerminal::Returnstat) | Symbol::Terminal(TokenType::KwBreak, _) => {
            let commandstat = self.children[0].visit(None);
            SemanticNode {
              children: SemanticNodeData::Statement {
                vardecl: None,
                atribstat: None,
                ifstat: None,
                forstat: None,
                statelist: None,
                commandstat: Some(Box::new(commandstat)),
              }
            }
          },
          // STATEMENT -> IFSTAT
          Symbol::NonTerminal(NonTerminal::Ifstat) => {
            SemanticNode {
              children: SemanticNodeData::Statement {
                vardecl: None,
                atribstat: None,
                ifstat: Some(Box::new(self.children[0].visit(None))),
                forstat: None,
                statelist: None,
                commandstat: None,
              }
            }
          },
          // STATEMENT -> FORSTAT
          Symbol::NonTerminal(NonTerminal::Forstat) => {
            SemanticNode {
              children: SemanticNodeData::Statement {
                vardecl: None,
                atribstat: None,
                ifstat: None,
                forstat: Some(Box::new(self.children[0].visit(None))),
                statelist: None,
                commandstat: None,
              }
            }
          },
          // STATEMENT -> lbrace STATELIST rbrace
          Symbol::Terminal(TokenType::Lbrace, _) => {
            let statelist = self.children[1].visit(None);
            SemanticNode {
              children: SemanticNodeData::Statement {
                vardecl: None,
                atribstat: None,
                ifstat: None,
                forstat: None,
                statelist: Some(Box::new(statelist)),
                commandstat: None,
              }
            }
          },
          // STATEMENT -> semicolon
          Symbol::Terminal(TokenType::Semicolon, _) => {
            SemanticNode {
              children: SemanticNodeData::Statement {
                vardecl: None,
                atribstat: None,
                ifstat: None,
                forstat: None,
                statelist: None,
                commandstat: None,
              }
            }
          },
          _ => panic!()
        }
      },
      Symbol::NonTerminal(NonTerminal::Vardecl) => {
        // VARDECL -> var_type id CONST_INDEX
        let const_index = if self.children[2].children.len() == 0 {
          None
        } else {
          Some(Box::new(self.children[2].visit(None)))
        };
        SemanticNode {
          children: SemanticNodeData::Vardecl {
            var_type: Box::new(self.children[0].visit(None)),
            id: Box::new(self.children[1].visit(None)),
            const_index
          }
        }
      },
      Symbol::NonTerminal(NonTerminal::ConstIndex) => {
        match self.children.len() {
          // CONST_INDEX -> lbracket const_int rbracket CONST_INDEX
          4 => {
            let mut new_inh = match inh {
              None => vec![],
              Some(inh) => inh.clone(),
            };
            new_inh.push(self.children[1].visit(None));
            self.children[3].visit(Some(&new_inh))
          },
          // CONST_INDEX -> ''
          0 => {
            SemanticNode {
              children: SemanticNodeData::ConstIndex { index: inh.unwrap().clone() },
            }
          },
          _ => panic!()
        }
      },
      Symbol::NonTerminal(NonTerminal::VarIndex) => {
        match self.children.len() {
          // VAR_INDEX -> ''
          0 => {
            SemanticNode {
              children: SemanticNodeData::VarIndex { index: inh.unwrap().clone() }
            }
          },
          // VAR_INDEX -> lbracket NUMEXPRESSION rbracket VAR_INDEX
          4 => {
            let mut new_params = match inh {
              None => vec![],
              Some(inh) => inh.clone()
            };
            new_params.push(self.children[1].visit(None));
            self.children[1].visit(Some(&new_params))
          },
          _ => panic!()
        }
      }, 
      // ATRIBSTAT -> LVALUE op_assign ATRIBSTATEVALUE
      Symbol::NonTerminal(NonTerminal::Atribstat) => {
        SemanticNode {
          children: SemanticNodeData::Atribstat {
            lvalue: Box::new(self.children[0].visit(None)),
            value: Box::new(self.children[2].visit(None))
          }
        }
      },
      Symbol::NonTerminal(NonTerminal::Atribstatevalue) => {
        match self.children[0].value {
          Symbol::NonTerminal(NonTerminal::Expression) => {
            SemanticNode {
              children: SemanticNodeData::Atribstatevalue { expression: Some(Box::new(self.children[0].visit(None))), allocexpression: None, funccall: None }
            }
          },
          Symbol::NonTerminal(NonTerminal::Allocexpression) => {
            SemanticNode {
              children: SemanticNodeData::Atribstatevalue { expression: None, allocexpression: Some(Box::new(self.children[0].visit(None))), funccall: None }
            }
          },
          Symbol::NonTerminal(NonTerminal::Funccall) => {
            SemanticNode {
              children: SemanticNodeData::Atribstatevalue { expression: None, allocexpression: None, funccall: Some(Box::new(self.children[0].visit(None))) }
            }
          },
          _ => panic!()
        }
      },
      // FUNCCALL -> func_id lparenthesis PARAMLISTCALL rparenthesis
      Symbol::NonTerminal(NonTerminal::Funccall) => {
        SemanticNode {
          children: SemanticNodeData::Funccall {
            id: Box::new(self.children[0].visit(None)),
            paramlistcall: if self.children[1].children.len() > 0 {
              Some(Box::new(self.children[2].visit(None)))
            } else {
              None
            }
          }
        }
      }, 
      Symbol::NonTerminal(NonTerminal::Paramlistcall) => {
        match self.children.len() {
          // PARAMLISTCALL -> expression PARAMLISTCALL1
          2 => {
            let inh = vec![self.children[0].visit(None)];
            self.children[1].visit(Some(&inh))
          },
          _ => panic!()
        }
      }, 
      Symbol::NonTerminal(NonTerminal::Paramlistcall1) => {
        match self.children.len() {
          // PARAMLISTCALL_1 -> ''
          0 => {
            SemanticNode {
              children: SemanticNodeData::Paramlistcall { paramlist: inh.unwrap().clone() },
            }
          },
          // PARAMLISTCALL_1 -> comma id PARAMLISTCALL_1
          3 => {
            let mut new_params = match inh {
              None => vec![],
              Some(inh) => inh.clone(),
            };
            new_params.push(self.children[1].visit(None));
            self.children[1].visit(Some(&new_params))
          },
          _ => panic!()
        }
      },
      // PRINTSTAT -> kw_print EXPRESSION 
      Symbol::NonTerminal(NonTerminal::Printstat) => {
        if self.children.len() != 2 { panic!() }
        SemanticNode {
          children: SemanticNodeData::Printstat { expression: Box::new(self.children[0].visit(None)) },
        }
      },
      // READSTAT -> kw_read LVALUE
      Symbol::NonTerminal(NonTerminal::Readstat) => {
        if self.children.len() != 2 { panic!() }
        SemanticNode {
          children: SemanticNodeData::Readstat { lvalue: Box::new(self.children[0].visit(None)) },
        }
      }, 
      Symbol::NonTerminal(NonTerminal::Returnstat) => {
        let Symbol::Terminal(_, token) = &self.children[0].value else { panic!("Expected terminal token for return statement"); };
        SemanticNode {
          children: SemanticNodeData::Returnstat {
            token: token.clone().unwrap()
          },
        }
      },
      // IFSTAT -> kw_if lparenthesis EXPRESSION rparenthesis lbrace STATELIST rbrace ELSESTAT 
      Symbol::NonTerminal(NonTerminal::Ifstat) => {
        if self.children.len() != 8 { panic!() }
        SemanticNode {
          children: SemanticNodeData::Ifstat {
            condition: Box::new(self.children[2].visit(None)),
            then_branch: Box::new(self.children[5].visit(None)),
            else_branch: if self.children[7].children.len() > 0 {
              Some(Box::new(self.children[7].visit(None)))
            } else {
              None
            }
          },
        }
      }, 
      Symbol::NonTerminal(NonTerminal::Elsestat) => {
        // ELSESTAT -> kw_else ELSESTAT_1
        if self.children.len() != 2 { panic!() }
        SemanticNode {
          children: SemanticNodeData::Elsestat {
            statement: Box::new(self.children[1].visit(None)),
          }
        }
      }, 
      Symbol::NonTerminal(NonTerminal::Elsestat1) => {
        match self.children.len() {
          // ELSESTAT_1 -> lbrace STATELIST rbrace
          3 => { self.children[1].visit(None) },
          // ELSESTAT_1 -> IFSTAT
          1 => { self.children[0].visit(None) },
          _ => panic!()
        }
      },
      // FORSTAT -> kw_for lparenthesis ATRIBSTAT semicolon EXPRESSION semicolon ATRIBSTAT rparenthesis lbrace STATELIST rbrace
      Symbol::NonTerminal(NonTerminal::Forstat) => {
        if self.children.len() != 11 { panic!() }
        SemanticNode {
          children: SemanticNodeData::Forstat {
            init: Box::new(self.children[2].visit(None)),
            condition: Box::new(self.children[4].visit(None)),
            increment: Box::new(self.children[6].visit(None)),
            body: Box::new(self.children[9].visit(None)),
          },
        }
      }, 
      Symbol::NonTerminal(NonTerminal::Allocexpression) => {
        if self.children.len() != 3 { panic!() }
        SemanticNode {
          children: SemanticNodeData::Allocexpression {
            var_type: Box::new(self.children[1].visit(None)),
            dimensions: Box::new(self.children[2].visit(None)),
          },
        }
      },
      // Aqui começam as regras semânticas para construção da AST
      // Parte das regras anteriores aplicaram o mesmo conceito de forma a otimizar o processo de compilação
      // Porém, como apenas essa parte estava descrita no enunciado, as regras semânticas a seguir são mais específicas
      
      // EXPRESSION -> NUMEXPRESSION EXPRESSION_1
      //  EXPRESSION_1.inh = NUMEXPRESSION.ptr
      //  EXPRESSION.ptr = EXPRESSION_1.ptr
      Symbol::NonTerminal(NonTerminal::Expression) => {
        let inh = self.children[0].visit(None);
        self.children[1].visit(Some(&vec![inh]))
      }, 
      Symbol::NonTerminal(NonTerminal::Expression1) => {
        match self.children.len() {
          // EXPRESSION_1 -> OP_EXPRESSION NUMEXPRESSION
          //  EXPRESSION_1.ptr = Node(EXPRESSION, Some(vec![EXPRESSION_1.inh, OP_EXPRESSION.ptr, NUMEXPRESSION.ptr]))
          2 => {
            SemanticNode {
              children: SemanticNodeData::Expression { 
                numexpression: Box::new(inh.unwrap()[0].clone()),
                op_expression: Some(Box::new(self.children[0].visit(None))),
                numexpression2: Some(Box::new(self.children[1].visit(None))),
              },
            }
          }
          // EXPRESSION_1 -> ''
          // EXPRESSION_1.ptr = EXPRESSION_1.inh
          0 => {
            SemanticNode {
              children: SemanticNodeData::Expression { 
                numexpression: Box::new(inh.unwrap()[0].clone()),
                op_expression: None,
                numexpression2: None,
              },
            }
          },
          _ => panic!()
        }
      }, 
      Symbol::NonTerminal(NonTerminal::Numexpression) => {
        // NUMEXPRESSION -> TERM NUMEXPRESSION_1
        //  NUMEXPRESSION_1.inh = [TERM.ptr]
        //  NUMEXPRESSION.ptr = NUMEXPRESSION_1.ptr
        let inh = self.children[0].visit(None);
        self.children[1].visit(Some(&vec![inh]))
      }, 
      Symbol::NonTerminal(NonTerminal::Numexpression1) => {
        match self.children.len() {
        // NUMEXPRESSION_1 -> OP_NUMEXPRESSION TERM NUMEXPRESSION_1
        //  NUMEXPRESSION_1_1.inh = Node(NUMEXPRESSION, vec![NUMEXPRESSION_1.inh.children[0], OP_NUMEXPRESSION.ptr, TERM.ptr])
        //  NUMEXPRESSION_1.ptr = NUMEXPRESSION_1_1.ptr
          3 => {
            let left_size = SemanticNode {
              children: SemanticNodeData::Numexpression { 
                term: Box::new(inh.unwrap()[0].clone()),
                op_numexpression: Some(Box::new(self.children[0].visit(None))),
                term2: Some(Box::new(self.children[1].visit(None))),
              },
            };
            self.children[2].visit(Some(&vec![left_size]))
          }
          // NUMEXPRESSION_1 -> ''
          //  NUMEXPRESSION_1.ptr = NUMEXPRESSION_1.inh
          0 => {
            // Checa se o nodo herdado é do tipo Numexpression ou Term
            match inh.unwrap()[0].children {
              // Se for do tipo Numexpression, retorna o nodo herdado
              SemanticNodeData::Numexpression { .. } => { inh.unwrap()[0].clone() }
              // Se for do tipo Term, retorna um novo nodo Numexpression com o termo herdado
              SemanticNodeData::Term { .. } => {
                SemanticNode {
                  children: SemanticNodeData::Numexpression { 
                    term: Box::new(inh.unwrap()[0].clone()),
                    op_numexpression: None,
                    term2: None,
                  },
                }
              }
              _ => panic!()
            }
          },
          _ => panic!()
        }
      }, 
      Symbol::NonTerminal(NonTerminal::Term) => {
        // TERM -> FACTOR TERM_1
        //  TERM_1.inh = [FACTOR.ptr]
        //  TERM.ptr = TERM_1.ptr
        let inh = self.children[0].visit(None);
        self.children[1].visit(Some(&vec![inh]))
      }, 
      Symbol::NonTerminal(NonTerminal::Term1) => {
        match self.children.len() {
          // TERM_1 -> OP_TERM FACTOR TERM_1
          //  TERM_1_1.inh = Node(TERM, vec![TERM_1.inh.children[0], OP_TERM.ptr, FACTOR.ptr])
          //  TERM_1.ptr = TERM_1_1.ptr
          3 => {
            let left_size = SemanticNode {
              children: SemanticNodeData::Term { 
                factor: Box::new(inh.unwrap()[0].clone()),
                op_term: Some(Box::new(self.children[0].visit(None))),
                factor2: Some(Box::new(self.children[1].visit(None))),
              },
            };
            self.children[2].visit(Some(&vec![left_size]))
          }
          // TERM_1 -> ''
          //  TERM_1.ptr = TERM_1.inh
          0 => {
            // Checa se o nodo herdado é do tipo Term ou Factor
            match inh.unwrap()[0].children {
              // Se for do tipo Term, retorna o nodo herdado
              SemanticNodeData::Term { .. } => { inh.unwrap()[0].clone() }
              // Se for do tipo Unaryexpression, retorna um novo nodo Term com o fator herdado
              SemanticNodeData::Unaryexpression { .. } => {
                SemanticNode {
                  children: SemanticNodeData::Term { 
                    factor: Box::new(inh.unwrap()[0].clone()),
                    op_term: None,
                    factor2: None,
                  },
                }
              }
              _ => panic!()
            }
          },
          _ => panic!()
        }
      }, 
      Symbol::NonTerminal(NonTerminal::Unaryexpression) => {
        match self.children.len() {
          // UNARYEXPRESSION -> FACTOR
          //  UNARYEXPRESSION.ptr = FACTOR.ptr
          1 => {
            SemanticNode {
              children: SemanticNodeData::Unaryexpression { 
                op: None,
                factor: Box::new(self.children[0].visit(None))
              },
            }
          },
          // UNARYEXPRESSION -> OP_NUMEXPRESSION FACTOR
          2 => {
            SemanticNode {
              children: SemanticNodeData::Unaryexpression { 
                op: Some(Box::new(self.children[0].visit(None))),
                factor: Box::new(self.children[1].visit(None))
              },
            }
          },
          _ => panic!()
        }
      }, 
      Symbol::NonTerminal(NonTerminal::Factor) => {
        match self.children[0].value {
          // FACTOR -> lparenthesis EXPRESSION rparenthesis
          Symbol::Terminal(TokenType::Lparenthesis, _) => {
            SemanticNode {
              children: SemanticNodeData::Factor { 
                expression: Some(Box::new(self.children[1].visit(None))),
                lvalue: None,
                constant: None,
              },
            }
          },
          // FACTOR -> LVALUE
          Symbol::NonTerminal(NonTerminal::Lvalue) => {
            SemanticNode {
              children: SemanticNodeData::Factor { 
                expression: None,
                lvalue: Some(Box::new(self.children[0].visit(None))),
                constant: None,
              },
            }
          },
          // FACTOR -> CONSTANT
          Symbol::NonTerminal(NonTerminal::Constant) => {
            SemanticNode {
              children: SemanticNodeData::Factor { 
                expression: None,
                lvalue: None,
                constant: Some(Box::new(self.children[0].visit(None))),
              },
            }
          },
          _ => panic!()
        }
      }, 
      Symbol::NonTerminal(NonTerminal::Lvalue) => {
        if self.children.len() != 2 { panic!() }
        SemanticNode {
          children: SemanticNodeData::Lvalue { 
            id: Box::new(self.children[0].visit(None)),
            var_index: if self.children[1].children.len() > 0 {
              Some(Box::new(self.children[1].visit(None)))
            } else {
              None
            },
          },
        }
      }, 
      Symbol::NonTerminal(NonTerminal::Constant) => {
        let Symbol::Terminal(_token_type, token ) = self.children[0].clone().value else { panic!(); };
        SemanticNode {
          children: SemanticNodeData::Constant { 
            value: token.unwrap().value.unwrap(),
          },
        }
      }, 
      Symbol::NonTerminal(NonTerminal::OpExpression) => {
        let Symbol::Terminal(token_type, _) = self.children[0].value else { panic!(); };
        SemanticNode {
          children: SemanticNodeData::OpExpression {
            op: token_type,
          }
        }
      }, 
      Symbol::NonTerminal(NonTerminal::OpNumexpression) => {
        let Symbol::Terminal(token_type, _) = self.children[0].value else { panic!(); };
        SemanticNode {
          children: SemanticNodeData::OpNumexpression {
            op: token_type,
          }
        }
      }, 
      Symbol::NonTerminal(NonTerminal::OpTerm) => {
        let Symbol::Terminal(token_type, _) = self.children[0].value else { panic!(); };
        SemanticNode {
          children: SemanticNodeData::OpTerm {
            op: token_type,
          }
        }
      }, 
    }
  }
  fn to_string(&self, count: &mut u32) -> String {
    let mut result = String::new();
    let node_name = format!("{:?}_{}", self.value, count);
    *count += 1;
    match &self.value {
      Symbol::Terminal(token, _) => {
        result.push_str(&format!("  {} [label=\"{:?}\" color=\"blue\"]\n", node_name, token));
      },
      Symbol::NonTerminal(nt) => {
        result.push_str(&format!("  {} [label=\"{:?}\" color=\"green\"]\n", node_name, nt));
      },
    }
    match &self.value {
      Symbol::Terminal(..) => {},
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
  pub fn new() -> Result<Self, Box<dyn Error>> {
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
          if let Ok(token) = TokenType::from_str(s) { Symbol::Terminal(token, None) }
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
    let root = Node::new( 
      Symbol::NonTerminal(NonTerminal::Program),
      Rc::clone(&parse_table),
      Rc::clone(&rules),
      Rc::new(ScopeStack::new()),
    );
    Ok(SyntaxTree { root })
  }
  pub fn parse(&mut self, tokens: &Vec<Token>) -> Result<(), Box<dyn Error>> {
    self.root.parse(tokens, &mut 0)?;
    Ok(())
  }
  pub fn save(&self, path: &str) -> Result<(), Box<dyn Error>> {
    let mut file = std::fs::File::create(path)?;
    writeln!(file, "// Visualize a árvore colando este arquivo em https://dreampuf.github.io/GraphvizOnline/?engine=dot")?;
    writeln!(file, "digraph G {{")?;
    writeln!(file, "{}", self.root.to_string(&mut 0))?;
    writeln!(file, "}}")?;
    Ok(())
  }
  pub fn semantic_tree(&mut self) -> Result<SemanticTree, Box<dyn Error>> {
    // TODO
    let semantic_tree = SemanticTree {
      root: self.root.visit(None),
      scopes: ScopeStack::new(),
    };
    Ok(semantic_tree)
  }
}
