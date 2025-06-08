import json

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

print("Não terminais:", ", ".join(sorted(variables)))
print("Terminais:", ", ".join(sorted(terminals)))
print()
automata = set([token[0] for token in tokens])

# Check if all tokens in syntax have a representation in tokens.json
print("Undefined tokens:", ", ".join(sorted(terminals.difference(automata))))
print()
# Warn unused tokens in tokens.json
print("Unused tokens:", ", ".join(sorted(automata.difference(terminals))))


