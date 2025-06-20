use std::collections::HashMap;
use std::error::Error;

struct SymbolEntry {
  appearances: Vec<usize>,
}

type Scope = HashMap<String, SymbolEntry>;

pub struct ScopeStack {
  stack: Vec<Scope>,
}


impl ScopeStack {
  pub fn new() -> Self {
    ScopeStack { stack: vec![HashMap::new()] }
  }

  pub fn push_scope(&mut self) {
    self.stack.push(HashMap::new());
  }

  pub fn pop_scope(&mut self) -> Option<Scope> {
    self.stack.pop()
  }

  pub fn insert_symbol(&mut self, name: String, entry: SymbolEntry) -> Result<(), Box<dyn Error>> {
    // A pilha de escopo sempre deve ter pelo menos um escopo, então stack.last_mut() nunca deve retornar None.
    let Some(current_scope) = self.stack.last_mut() else { return Err("No current scope to insert symbol".into()); };
    // Se o escopo atual já contém o símbolo, retorna erro semântico de redefinição de símbolo.
    if current_scope.contains_key(&name) {
      return Err(format!("Erro semântico: Redefinição do símbolo '{}' ", name).into());
    }
    current_scope.insert(name, entry);

    Ok(())
  }

  pub fn get_symbol(&self, name: &str) -> Option<&SymbolEntry> {
    // Procura o símbolo nos escopos, começando do mais interno (topo da pilha).
    for scope in self.stack.iter().rev() {
      if let Some(entry) = scope.get(name) {
        return Some(entry);
      }
    }
    None
  }
}