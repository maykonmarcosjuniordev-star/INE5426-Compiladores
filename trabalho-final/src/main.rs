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
  let mut lexer = Lexer::new();
  lexer.parse(&input)?;
  lexer.save_token_list("output/token_list.txt")?;
  lexer.save_token_table("output/token_table.txt")?;

  // Syntax analysis
  let mut syntax_tree = SyntaxTree::new()?;
  syntax_tree.parse(&lexer.token_list)?;
  syntax_tree.save("output/parse_tree.txt")?;

  // Semantic analysis
  let mut semantic_tree = syntax_tree.semantic_tree()?;
  semantic_tree.save("output/ast.txt")?;
  let expression_trees = semantic_tree.create_expression_trees();
  for (i, tree) in expression_trees.iter().enumerate() {
    tree.save(&format!("expression_trees/tree_{}.dot", i));
  }
  semantic_tree.semantic_analysis()?;


  // Should run without errors, except of course if the output file can't be created
  // semantic_tree.generate_code("output/intermediate_code.txt")?;

  Ok(())
}
