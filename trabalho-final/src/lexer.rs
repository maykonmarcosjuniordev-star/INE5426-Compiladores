use crate::fda::FDA;
use crate::token::Token;
use crate::grammar::{token_type::TokenType, const_type::ConstType};
use std::collections::HashMap;
use std::error::Error;
use std::io::Write;

type State = u32;
pub struct Lexer {
  pub fda: FDA,
  pub token_list: TokenList,
  pub token_table: TokenTable,
  line_count: usize,
  column_count: usize,
  token_value: String,
  string: bool,
  current_state: State
}

pub type TokenList = Vec<Token>;
pub type TokenTable = HashMap<String, TokenEntry>;
pub type TokenEntry = Vec<(u32, u32)>;

impl Lexer {
  pub fn new() -> Lexer {
    let fda = FDA::from_file().expect("Lexer automata file not found");
    Lexer { 
      fda,
      token_list: vec![],
      token_table: HashMap::new(),
      line_count: 1,
      column_count: 0,
      token_value: String::new(),
      string: false,
      current_state: 0
    }
  }

  /// Retorna um erro léxico com a linha, coluna e o token inválido.
  fn lexical_error(&self) -> Result<(), Box<dyn Error>> {
    Err(format!("Erro léxico: Caracter inválido na linha {}, coluna {}: '{}'", self.line_count, self.column_count, self.token_value).into())
  }

  /// Verifica se o token construído até agora é válido.
  /// Se for, cria um token com o tipo e valor do token encontrado até agora,
  /// além da linha e coluna onde o token foi encontrado.
  fn is_valid_token(&mut self) -> Result<(), Box<dyn Error>> {
    match self.fda.token_table.get(&self.current_state) {
      // Se o estado atual for um estado final, significa que um token válido foi encontrado
      // -> e que o caractere atual é o início de um novo token possível
      Some(token_type) => {
        // Cria um token com o tipo e valor do token encontrado até agora,
        // além da linha e coluna onde o token foi encontrado
        let token = Token {
          token_type: *token_type,
          value: if token_type.has_value() {Some(ConstType::from_str(&self.token_value))} else { None },
          line: self.line_count,
          column: self.column_count-self.token_value.len(),
        };
        // Se for um identificador, adiciona-o à tabela de símbolos
        if token_type.is_id() {
          let entry = self.token_table.get_mut(&self.token_value);
          match entry {
            Some(e) => {
              // Se o identificador já existir na tabela de símbolos, adiciona a linha e coluna onde foi encontrado
              e.push((token.line as u32, token.column as u32));
            },
            None => {
            // Caso contrário, insere o identificador na tabela de símbolos
            // TODO: Isso poderia ser feito com uma função auxiliar token.parse()?
              self.token_table.insert(
                self.token_value.clone(),
                vec![(token.line as u32, token.column as u32)]
                );
            }
          }
        }
        // Armazena o token encontrado na lista de tokens
        self.token_list.push(token);
      },
      // Se não for, isso significa que o token construído até agora é inválido e deve ser descartado
      None => {
        if !self.token_value.is_empty() {
          return self.lexical_error();
        }
      }
    }
    Ok(())
  }

  /// Transita pelo autômato finito determinístico (AFD) com o estado atual e o caractere fornecido.
  /// Atualisa o estado atual e o valor do token (se must_push for true), se a transição for válida.
  /// Retorna true se a transição for válida, false caso contrário.
  fn transition(&mut self, state: State, character: char) -> bool {
    let Some(next_state) = self.fda.transition(state, character) else { return false; };
    // Se a transição for válida, atualiza o estado atual e adiciona o caractere ao valor do token
    self.current_state = *next_state;
    // Ignora caracteres em branco fora de strings
    if !character.is_whitespace() || self.current_state != self.fda.initial_state { 
      self.token_value.push(character);
    }
    return true;
  }

  /// Realiza a análise léxica do input fornecido.
  /// Lê o input caractere por caractere, atualizando o estado do autômato e construindo tokens válidos.
  /// Se encontrar um erro léxico, retorna um erro.
  pub fn parse(&mut self, input: &str) -> Result<(), Box<dyn Error>> {
    // Para cada caractere do input, realiza a análise léxica
    for char in input.chars() {
      // contagem da coluna onde o caractere está
      self.column_count += 1;
      // Abrindo e fechando strings
      if char == '"' {
        self.string = !self.string;
      }
      // Se o caractere for um espaço em branco, ignore-o,
      // Mas só se estivermos no estado inicial do autômato
      // -> Isso permite ignorar vários espaços em branco seguidos; algo como em "return       ;"
      else if self.current_state == self.fda.initial_state && char.is_whitespace() { 
        // Se for uma quebra de linha, incrementa a contagem de linhas e reseta a contagem de colunas
        if char == '\n' {
          self.line_count += 1;
          self.column_count = 0;
        }
        continue;
      }
      // A linguagem é case-insensitive (fora de uma string),
      let character = if !self.string && char.is_alphabetic() {
        // então converte o caractere para minúsculo
        char.to_ascii_lowercase()
      } else {
        char
      };
      
      if !self.transition(self.current_state, character) {
        // Se a transição não for válida, verifica se o estado atual é um estado final
        self.is_valid_token()?;
        // Reseta o token encontrado até agora
        self.token_value.clear();
        // Verifica se o caractere atual é um possível início de token
        if !self.transition(self.fda.initial_state, character) {
          // Se não for, retorna um erro léxico
          // Já que a compilação para no primeiro erro, não precisa resetar o estado atual
          return self.lexical_error();
        }
      }
      if char == '\n' {
        self.line_count += 1;
        self.column_count = 0;
      }
    }
    // Depois de ler todo o input, verifica se o último token lido é válido
    self.is_valid_token()?;
    // Adiciona um token de fim de arquivo (EOF) à lista de tokens
    self.token_list.push(Token{
      token_type: TokenType::Eof,
      value: None,
      line: self.line_count,
      column: self.column_count,
    });

    println!("Análise léxica concluída com sucesso, {} tokens no total.", self.token_list.len());
    Ok(())
  }

  pub fn save_token_list(&self, path: &str) -> Result<(), Box<dyn Error>> {
    let mut file = std::fs::File::create(path)?;
    for token in &self.token_list {
      writeln!(file, "{:?}", token)?;
    }
    println!("Lista de tokens salva em {}", path);
    Ok(())
  }

  pub fn save_token_table(&self, path: &str) -> Result<(), Box<dyn Error>> {
    let mut file = std::fs::File::create(path)?;
    for (key, value) in &self.token_table {
      writeln!(file, "{}: {:?}", key, value)?;
    }
    println!("Tabela de símbolos salva em {}", path);
    Ok(())
  }
}