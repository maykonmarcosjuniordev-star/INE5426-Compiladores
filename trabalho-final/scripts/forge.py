prefix = """
# Esse arquivo está no formato de entrada que o site https://jsmachines.sourceforge.net/machines/ll1.html pede para gerar a tabela LL1 de uma gramática
# Modificações em relação à gramática original além de fatoração:
# - Substituição do terminal id para identificação da função por um func_id. A definição desse token é a mesma que um id normal, porém com um @ no começo.
# - Obrigatoriedade de chaves para blocos condicionais para remover a ambiguidade do caso if statement if statement else statement.
# Além disso, a gramática inteira foi feita de forma a evitar repetição desnecessária,
# Por exemplo: A -> A + A | A - A foi transformado em A -> A OP A; OP -> + | -
# Detalhe da análise léxica: A linguagem é case insentive.

"""[1:]


with open("grammars/syntax.txt") as f: lines = f.readlines()

with open("grammars/syntax-forge.txt", "w") as f:
  f.write(prefix)
  for line in lines:
    if line.startswith("#"): continue
    head, body = line.split(",")
    f.write(f"{head.strip()} -> {body.strip()}\n")
