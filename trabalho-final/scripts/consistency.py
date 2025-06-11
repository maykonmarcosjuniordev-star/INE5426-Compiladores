import json

VALUED_TOKENS = [
  "const_float",
  "const_int",
  "const_string",
  "func_id",
  "id",
  "var_type"
]

# Load syntax.txt
with open("grammars/syntax.txt") as f: syntax = f.readlines()
syntax = [line.strip() for line in syntax]
# Load tokens.json
with open("grammars/tokens.json") as f: tokens = json.load(f)

variables = set()
terminals = set()
for line in syntax:
  head, *body = line.split(",")
  variables.add(head)
for line in syntax:
  head, *body = line.split(",")
  for symbol in body:
    if symbol == "''": continue
    if symbol not in variables: terminals.add(symbol)

automata = set([token[0] for token in tokens])

# Check if all tokens in syntax have a representation in tokens.json
diff = terminals.difference(automata)
if diff: print("Undefined tokens:", ", ".join(sorted(diff)), '\n')
# Warn unused tokens in tokens.json
diff = automata.difference(terminals)
if diff: print("Unused tokens:", ", ".join(sorted(diff)), '\n')

# Create TokenType Enumerator
def clean_token(token: str) -> str:
  new_string = token[0].upper()
  x = False
  for char in token[1:]:
    if char == "_":
      x = True
      continue

    if x:
      new_string += char.upper()
      x = False
    else:
      new_string += char
  return new_string

with open("src/grammar/token_type.rs", "w") as f:
  f.write("use std::error::Error;\n\n")
  f.write("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]\n")
  f.write("pub enum TokenType {\n")
  for token in sorted(terminals):
    f.write(f"  {clean_token(token)},\n")
  f.write("  EOF,\n")
  f.write("}")
  f.write("\n\n")
  f.write("impl TokenType {\n")
  f.write("  pub fn from_str(s: &str) -> Result<TokenType, Box<dyn Error>> {\n")
  f.write("    match s {\n")
  for token in sorted(terminals):
    f.write(f"      \"{token}\" => Ok(TokenType::{clean_token(token)}),\n")
  f.write(f"      \"eof\" => Ok(TokenType::EOF),\n")
  f.write("      _ => Err(format!(\"Invalid TokenType: {}\", s).into())")
  f.writelines(["    }\n", "  }\n\n"])

  valued_string = " | ".join([f"TokenType::{clean_token(token)}" for token in VALUED_TOKENS])
  f.write("""  pub fn has_value(&self) -> bool {{
    match self {{
      {} => true,
      _ => false,
    }}
  }}\n""".format(valued_string))

  f.write("}\n")
