use std::collections::HashMap;
use std::error::Error;
use crate::grammar::const_type::VarType;
use std::fs::{File, OpenOptions}; // Import Write trait for writeln! macro
use std::io::Write; // Import Write trait for writeln! macro

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ScopeType {
  Function,
  Loop,
  LoopInit,
  If,
  Else,
  Any
}

#[derive(Debug, Clone, PartialEq)]
pub struct SymbolEntry {
  pub appearances: Vec<(usize, usize)>, // (line, column)
  pub var_type: Vec<VarType>,
  pub const_index: Vec<u32>,
}

type Scope = (ScopeType, HashMap<String, SymbolEntry>);

#[derive(Debug)]
pub struct ScopeStack {
  pub stack: Vec<Scope>,
  file: File,
}

impl ScopeStack {
  pub fn new() -> Self {
    ScopeStack { 
      stack: vec![(ScopeType::Any, HashMap::new())],
      file: OpenOptions::new()
        .create(true)
        .write(true)
        .open("output/scope_stack.log")
        .expect("Unable to open file"),
    }
  }

  pub fn push_scope(&mut self, scope_type: ScopeType) {
    self.stack.push((scope_type, HashMap::new()));
  }

  pub fn pop_scope(&mut self) -> Option<Scope> {
    let x = self.stack.pop();
    let mut scope_display = String::new();
    if let Some((scope_type, table)) = &x {
      scope_display.push_str(&format!("Escopo: {:?}", scope_type));
      for (name, entry) in table {
        if entry.var_type.len() == 1 {
          scope_display.push_str(&format!("\n  Símbolo: {}, Tipo: {:?}, Índices: {:?}, Aparições: {:?}", name, entry.var_type[0], entry.const_index, entry.appearances));
        } else {
          scope_display.push_str(&format!("\n  Símbolo: {}, Tipo: {:?}, Índices: {:?}, Aparições: {:?}", name, entry.var_type, entry.const_index, entry.appearances));
        }
      }
    }
    writeln!(self.file, "{}\n", scope_display).unwrap();
    x
  }

  pub fn insert_symbol(&mut self, name: String, entry: SymbolEntry) -> Result<(), Box<dyn Error>> {
    // A pilha de escopo sempre deve ter pelo menos um escopo, então stack.last_mut() nunca deve retornar None.
    let Some(current_scope) = self.stack.last_mut() else { panic!("No current scope to insert symbol"); };
    // Se o escopo atual já contém o símbolo, retorna erro semântico de redefinição de símbolo.
    let table = &mut current_scope.1;
    if table.contains_key(&name) {
      let (line, column) = entry.appearances.last().unwrap_or(&(0, 0));
      return Err(format!("Erro semântico: Redefinição de símbolo na linha {}, coluna {}: '{}'", line, column, name).into());
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
    for scope in self.stack.iter_mut().rev() {
      if scope.1.contains_key(name) {
        scope.1.get_mut(name)
          .ok_or_else(|| format!("Erro semântico: Símbolo não encontrado na linha {}, coluna {}: {}", line, column, name))?
          .appearances.push((line, column));
        
        return Ok(());
      }
    }    
    
    Err(format!("Erro semântico: Símbolo não encontrado na linha {}, coluna {}: {}", line, column, name).into())
  }

  pub fn contains(&self, scope_type: ScopeType) -> bool {
    // Verifica se a pilha de escopo contém algum escopo do tipo especificado.
    self.stack.iter().rev().any(|(st, _)| *st == scope_type)
  }
}
