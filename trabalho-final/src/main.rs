mod token;
mod fda;
mod lexer;
mod syntax;
mod cfg;

use lexer::Lexer;
use syntax::SyntaxTree;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  // Read the file to be compiled from command line arguments
  let args: Vec<String> = std::env::args().collect();
  if args.len() < 2 { return Err("Usage: <input_file>".into()); }
  let input_file = &args[1];
  let input = std::fs::read_to_string(input_file)?;

  // Lexical analysis
  let lexer = Lexer::new();
  let token_list = lexer.parse(&input)?;

  // Syntax analysis
  let mut tree = SyntaxTree::new();
  // tree.parse(token_list);

  println!("{:?}", tree);
  Ok(())
}
