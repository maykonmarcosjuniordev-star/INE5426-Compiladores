mod token;
mod fda;
mod lexer;
mod syntax;
mod grammar;

use lexer::Lexer;
use syntax::SyntaxTree;

fn main() -> Result<(), Box<dyn std::error::Error>> {
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
  let mut tree = SyntaxTree::new()?;
  tree.parse(&lexer.token_list)?;
  tree.save("output/parse_tree.txt")?;
  Ok(())
}
