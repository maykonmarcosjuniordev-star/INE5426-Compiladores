# Esse script gera as estruturas relacionadas à gramática do compilador.
# Os arquivos de definição da gramática são lidos e usados para gerar os arquivos de código-fonte necessários.
# Arquivos lidos:
# - grammars/tokens.json
# - grammars/syntax.txt 
# Arquivos gerados:
# - src/grammar/token_type.rs
# - src/grammar/non_terminals.rs

# Os seguintes arquivos são templates para a geração dos arquivos de código-fonte:
# - scripts/token_type_template.txt
# - scripts/non_terminals_template.txt

import json

VALUED_TOKENS = [
  "const_float",
  "const_int",
  "const_string",
  "func_id",
  "id",
  "var_type"
]

ID_TOKENS = [
  "id",
  "func_id"
]

OPERATORS = [
  "op_eq",
  "op_ne",
  "op_gt",
  "op_ge",
  "op_lt",
  "op_le",
  "op_plus",
  "op_minus",
  "op_multiply",
  "op_division",
  "op_modular",
]

SCRIPT_NAME = "/".join(__file__.split("/")[-2:])

# Load syntax.txt
with open("grammars/syntax.txt") as f: syntax = [line.strip() for line in f.readlines()]
# Load tokens.json
with open("grammars/tokens.json") as f: tokens = json.load(f)

variables = set()
terminals = set()
# Constrói o conjunto de símbolos não terminais da sintaxe
for line in syntax:
  head, _body = line.split(",")
  variables.add(head)
# Constrói o conjunto de símbolos terminais da sintaxe
for line in syntax:
  head, body = line.split(",")
  body = body.split()
  for symbol in body:
    if symbol == "''": continue
    if symbol not in variables: terminals.add(symbol)

automata = set([token[0] for token in tokens])

# Verificar se todos os tokens presentes na sintaxe estão definidos em tokens.json
diff = terminals.difference(automata)
if diff: print("Undefined tokens:", ", ".join(sorted(diff)), '\n')
# Verificar se todos os tokens definidos em tokens.json estão sendo usados na sintaxe
# Isso não é um problema, é apenas boa prática garantir a paridade entre a análise léxica e sintática
diff = automata.difference(terminals)
if diff: print("Unused tokens:", ", ".join(sorted(diff)), '\n')

# Criar TokenType Enumerator
def clean_token(token: str) -> str:
  return token.replace("_", " ").title().replace(" ", "")

terminals.add("eof")  # Adiciona o token EOF para indicar o fim do arquivo
with open("scripts/token_type_template.txt") as f: token_type_template = f.read()
with open("src/grammar/token_type.rs", "w") as f:
  token_list = "  ".join([f"{clean_token(token)},\n" for token in sorted(terminals)])[:-1]
  token_string_list = "      ".join([f"\"{token}\" => Ok(TokenType::{clean_token(token)}),\n" for token in sorted(terminals)])[:-1]
  valued_string = " | ".join([f"TokenType::{clean_token(token)}" for token in VALUED_TOKENS])
  id_tokens = " | ".join([f"TokenType::{clean_token(token)}" for token in ID_TOKENS])
  operators = "      ".join([f"TokenType::{clean_token(token)} => Operator::{clean_token(token)[2:]},\n" for token in OPERATORS])
  f.write(token_type_template.format(token_list=token_list, token_string_list=token_string_list, valued_string=valued_string, id_tokens=id_tokens, script_name=SCRIPT_NAME, operators=operators))

# Criar NonTerminal enum
def clean_variable(var: str) -> str:
  return var.replace("_", " ").title().replace(" ", "")

non_terminal_list = "  ".join([f"{clean_variable(variable)},\n" for variable in sorted(variables)])[:-1]
non_terminal_string_list = "      ".join([f"\"{variable}\" => Ok(NonTerminal::{clean_variable(variable)}),\n" for variable in sorted(variables)])[:-1]
with open("scripts/non_terminals_template.txt") as f: non_terminal_template = f.read()
with open("src/grammar/non_terminals.rs", "w") as f:
  f.write(non_terminal_template.format(non_terminal_list=non_terminal_list, non_terminal_string_list=non_terminal_string_list, script_name=SCRIPT_NAME))
