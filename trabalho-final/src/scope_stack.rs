use std::collections::HashMap;
use std::error::Error;
use crate::grammar::const_type::VarType;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ScopeType {
  Function,
  Loop,
  Any
}

#[derive(Debug, Clone, PartialEq)]
pub struct SymbolEntry {
  pub appearances: Vec<(usize, usize)>, // (line, column)
  pub var_type: Vec<VarType>,
  pub const_index: Vec<u32>,
}

type Scope = (ScopeType, HashMap<String, SymbolEntry>);

#[derive(Debug, Clone, PartialEq)]
pub struct ScopeStack {
  pub stack: Vec<Scope>,
}

impl ScopeStack {
  pub fn new() -> Self {
    ScopeStack { stack: vec![(ScopeType::Any, HashMap::new())] }
  }

  pub fn push_scope(&mut self, scope_type: ScopeType) {
    self.stack.push((scope_type, HashMap::new()));
  }

  pub fn pop_scope(&mut self) -> Option<Scope> {
    self.stack.pop()
  }

  pub fn insert_symbol(&mut self, name: String, entry: SymbolEntry) -> Result<(), Box<dyn Error>> {
    // A pilha de escopo sempre deve ter pelo menos um escopo, então stack.last_mut() nunca deve retornar None.
    let Some(current_scope) = self.stack.last_mut() else { panic!("No current scope to insert symbol"); };
    // Se o escopo atual já contém o símbolo, retorna erro semântico de redefinição de símbolo.
    let table = &mut current_scope.1;
    if table.contains_key(&name) {
      return Err(format!("Erro semântico: Redefinição do símbolo '{}' ", name).into());
    }
    table.insert(name, entry);

    Ok(())
  }

  pub fn get_symbol(&self, name: &str) -> Option<SymbolEntry> {
    // Procura o símbolo nos escopos, começando do mais interno (topo da pilha).
    for scope in self.stack.iter().rev() {
      if let Some(entry) = scope.1.get(name) {
        return Some(entry.clone());
      }
    }
    None
  }

  pub fn count_appearance(&mut self, name: &str, line: usize, column: usize) -> Result<(), Box<dyn Error>> {
    // Conta as aparições do símbolo e adiciona a posição atual.
    if let Some(mut entry) = self.get_symbol(name) {
      entry.appearances.push((line, column));
      Ok(())
    } else {
      Err(format!("Erro semântico: símbolo '{}' não encontrado na linha {} coluna {}", name, line, column).into())
    }
  }

  pub fn contains(&self, scope_type: ScopeType) -> bool {
    // Verifica se a pilha de escopo contém algum escopo do tipo especificado.
    self.stack.iter().rev().any(|(st, _)| *st == scope_type)
  }
}