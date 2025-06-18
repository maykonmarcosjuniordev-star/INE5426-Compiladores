import json
from fda import FDA

VALID_LETTERS = { chr(i) for i in range(256) if chr(i).isalpha() and chr(i).islower() }
EMPTY_CHARS = { chr(i) for i in range(256) if chr(i).isspace() }

with open("grammars/tokens.json", "r") as f: token_types = json.load(f)
new_token_types = []
for token_type, token_data in token_types:
  if "string" in token_data and type(token_data["string"]) == list:
    for x in token_data["string"]:
      new_token_types.append((token_type, {"string": x}))
  else:
    new_token_types.append((token_type, token_data))
token_types = new_token_types
automata: list[tuple[str, FDA]] = []
for token_type, token_data in token_types:
  '''Cria um autômato finito determinístico para cada tipo de token.
  Alguns tokens são definidos apenas por uma string, como palavras reservadas. Nesse caso, basta criar uma cadeia de estados, uma para cada letra da string.
  Outros tokens já estão definidos como autômatos, para poupar o trabalho de parsear a regex que representa o token. Como identificadores ou números.
  Não sei ainda onde colocar essa informação. É útil pontuar que a união dos autômatos criará um autômato não determinístico, pois existem trechos em comum entre diferentes tokens, por exemplo toda palavra reservada pode ser apenas o começo de um identificador.
  Como o algoritmo de determinização de autômatos já está implementado, é importante que todo caso em que esse não determinismo ocorra, as transições sejam feitas de forma que a implementação do algoritmo de determinização consiga identificá-las e tratá-las.
  Basicamente isso é importante pois símbolos como "." serão utilizados para que a transição ocorra para _qualquer caracter_, mas quando existem outras transições saindo desse estadom, é importante que "." seja expandido para todas as transiçẽos de forma que o não determinismo seja percebido pelo algoritmo de determinização e tratado.
  No saco de strings e chars, é possível (e crucial*) utilizar o "." pois nenhum outro token segue o formato de começar com aspas (simples ou duplas). Porém, para um identificador, é preciso expandir "." em {a, b, c.. y, z}, pois outros tokens também podem ser aceitos por .*. Logo, há não determinismo que precisa ser tratado.
  * Motivo pelo qual o "." é crucial para o parsing de strings e chars, e porque outros tokens precisam utilizar a-z em vez de ".": "." poderia ser expandido para todas as letras do alfabeto, porém há de se considerar que em vez de usar apenas a tabela ASCII, espera-se que o lexer funcione para identificar qualquer string UTF, fazendo com que "." possa assumir milhares de valores, o que criaria um autômato desnecessariamente grande para parsear identificadores. Então, strings e chars usam "." para poder aceitar _qualquer_ valor UTF inserido pelo usuário, enquanto identificadores expandem n transições para cada letra aceita pela linguagem.
  Para expandir "." para todas as letras aceitas pela linguagem, irei usar or conjunto {chr(i) if chr(i).isalpha() and chr(i).islower() for i in range(256)}
  '''
  if "string" in token_data:
    # Create a state for each letter of the string
    string = token_data["string"]
    automaton = FDA()
    automaton.alphabet=frozenset(string)
    automaton.states=frozenset(frozenset((i,)) for i in range(len(string)+1))
    automaton.transitions=transitions = {frozenset((i,)): {string[i]: frozenset((frozenset((i+1,)),))} for i in range(len(string))}
    automaton.final_states=frozenset((frozenset((len(string),)),))
  else:
    # The automaton is already defined in the json file, just convert it to a fda
    final_states = {state for state in token_data["final_states"]}
    transitions = {}
    alphabet = set()
    for current_state, symbol, next_state in token_data["transitions"]:
      if current_state not in transitions: transitions[current_state] = {}
      # Tipos específicos de transições:
      # Apenas um símbolo: transição direta, nada a comentar
      if len(symbol) == 1:
        transitions[current_state][symbol] = next_state
        alphabet.add(symbol)
      # Contendo "-": transição de intervalo, cria uma transição para cada caracter no intervalo definido
      # O intervalo é definido pela tabela ASCII, a-c = 97-99, ou seja, 'a', 'b' e 'c'
      elif "-" in symbol and len(symbol) == 3:
        start, end = symbol.split("-")
        for i in range(ord(start), ord(end)+1):
          transitions[current_state][chr(i)] = next_state
          alphabet.add(chr(i))
      # Símbolos especiais que podem ser usados nos autômatos:
      # \c: Qualquer letra válida, definida nesse arquivo como qualquer caracter alfabético minúsculo.
      # como letras com acento também são válidas, não é equivalente a [a-z]
      elif symbol == "\\c":
        for i in VALID_LETTERS:
          transitions[current_state][i] = next_state
          alphabet.add(i)
      # \d: Qualquer dígito, de 0 a 9, equivalente a [0-9]
      elif symbol == "\\d":
        for i in range(10):
          transitions[current_state][str(i)] = next_state
          alphabet.add(str(i))
      # \.: Wildcard para caso o caracter lido não possua uma transição própria mas o estado possua uma transição genérica
      # O funcionamento está mais bem explicado na definição do autômato em src/fda.rs
      elif symbol == "\\.":
        transitions[current_state][chr(0)] = next_state
        alphabet.add(chr(0))
      else:
        raise ValueError(f"Invalid symbol {symbol} in transitions")

    transitions = {
      frozenset((c,)): {
        symbol: frozenset((frozenset((next_states, )),)) for symbol, next_states in next_states.items()
      } for c, next_states in transitions.items()
    }
    automaton = FDA()
    automaton.alphabet=frozenset(alphabet)
    states = set()
    for state, transtions in transitions.items():
      states.add(state)
      for next_states in transtions.values():
        states.update(next_states)
    automaton.states=frozenset(states)
    automaton.transitions=transitions
    automaton.final_states=frozenset(frozenset((i,)) for i in final_states)
  automaton.state_token_table = {state: token_type for state in automaton.final_states}
  automaton.initial_state = frozenset((0,))
  automata.append((token_type, automaton))

# Une todos os autômatos e depois determiniza esse autômato que deve reconhecer todos os tokens da linguagem.
mega_automaton = None
for token_type, automaton in automata:
  if mega_automaton is None: mega_automaton = automaton
  else: mega_automaton = mega_automaton.union(automaton)

mega_automaton = mega_automaton.deterministic_equivalent().enumerate_states()
# Manually add transitions to ignore empty spaces when the automaton is in the initial state; i.e., when the automaton is not in any token.
for char in EMPTY_CHARS: mega_automaton.transitions[mega_automaton.initial_state][char] = frozenset((mega_automaton.initial_state,))
mega_automaton.save("machines/lexer")
print("Lexer generated successfully.")
print(f"Lexer has {len(automata)} automata and {len(mega_automaton.final_states)} final states.")
print(f"Lexer has {len(mega_automaton.states)} states and {mega_automaton.transition_count()} transitions.")
