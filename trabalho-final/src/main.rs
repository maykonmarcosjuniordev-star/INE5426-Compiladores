/// Exercício-Programa INE5426
/// Trabalho Final - Compiladores
/// Participantes:
/// - Beatriz de Quadros Schmitt - 22100608
/// - Clara Rosa Oliveira Gonçalves - 22103511
/// - Gabriel Sartori Rangel - 22100617
/// - Mateus Goulart Chedid - 22100635
/// - Maykon Marcos Junior - 22102199

mod token;
mod fda;
mod lexer;
mod syntax;
mod grammar;
mod semantic;
mod expression;
mod scope_stack;
mod code_attrs;

use lexer::Lexer;
use syntax::SyntaxTree;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
  // Read the file to be compiled from command line arguments
  let args: Vec<String> = std::env::args().collect();
  if args.len() < 2 { return Err("Usage: <input_file>".into()); }
  let input_file = &args[1];
  let input = std::fs::read_to_string(input_file)?;

  // Lexical analysis
  let mut output = String::new();
  output.push_str(&format!("# INICIANDO ANÁLISE LÉXICA #"));
  let mut lexer = Lexer::new();
  lexer.parse(&input)?;
  lexer.output_stats(&mut output);

  // Syntax analysis
  output.push_str(&format!("\n# INICIANDO ANÁLISE SINTÁTICA #"));
  let mut syntax_tree = SyntaxTree::new()?;
  syntax_tree.parse(&lexer.token_list)?;
  syntax_tree.output_stats(&mut output);

  // Semantic analysis
  output.push_str(&format!("\n# INICIANDO ANÁLISE SEMÂNTICA #"));
  let mut semantic_tree = syntax_tree.semantic_tree()?;
  semantic_tree.semantic_analysis()?;
  semantic_tree.output_stats(&mut output);

  // Generate intermediate code
  output.push_str(&format!("\n# GERANDO CÓDIGO INTERMEDIÁRIO #"));
  let intermediate_code = semantic_tree.generate_code();
  output.push_str(&format!("Código intermediário gerado:\n{}", intermediate_code));

  // print the output
  println!("{}", output);

  Ok(())
}
