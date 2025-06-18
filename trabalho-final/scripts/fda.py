import math
State = frozenset[int]

class FDA:
  '''Classe que representa um autômato finito determinístico ou não determinístico.'''
  '''Esse código foi feito originalmente para a matéria de Linguagens Formais, mas foi adaptado para ser usado na análise léxica do compilador.'''
  '''Cada autômato é usado para representar um token, todo autômato possui uma tabela associando seus estados finais a qual token cada um representa.'''
  '''Inicialmente essa tabela é preenchida com o mesmo token, porém após algoritmos de união dos vários autômatos, essa tabela será útil para identificar qual o token lido pelo autômato.'''
  def __init__(self) -> None:
    self.initial_state: State = None
    self.transitions: dict[State, dict[str, frozenset[State]]] = {}
    self.final_states: frozenset[State] = frozenset()
    self.states: frozenset[State] = frozenset((frozenset(("qm",)),))
    self.num_states: int = 0
    self.alphabet: set[chr] = set()
    self.state_token_table: dict[State, str] = {}
    if self.initial_state is not None: self.enumerate_states()

  def transition_count(self) -> int:
    '''Retorna o número de transições do autômato.'''
    count = 0
    for state in self.transitions:
      for symbol in self.transitions[state]:
        count += len(self.transitions[state][symbol])
    return count

  def is_deterministic(self):
    '''Busca por transições por ε ou por um estado que tenha mais de um destino para um mesmo símbolo.'''
    for state in self.transitions:
      if "" in self.transitions[state]: return False
      for symbol in self.alphabet:
        if symbol not in self.transitions[state]: continue
        if len(self.transitions[state][symbol]) > 1: return False
    return True

  def enumerate_states(self) -> 'FDA':
    '''Converte todos os estados do autômato em ints de 0 a n-1, sendo n o número de estados do autômato.'''
    '''Exemplo: um autômato com 3 estados {"A", "B", "C"} vira {"0", "1", "2"}'''
    '''O estado inicial é sempre 0, os demais são ordenaos pelo nome do estado.'''
    non_initial_states = self.states.difference(frozenset((self.initial_state,)))
    states = {
      **{self.initial_state: 0},
      **{state: i+1 for i, state in enumerate(sorted(non_initial_states, key=lambda x: min(x)))}
    }
    self.states = frozenset(frozenset((states[state],)) for state in self.states)
    self.initial_state = frozenset((states[self.initial_state],))
    self.final_states = frozenset(frozenset((states[state],)) for state in self.final_states)
    new_transtions = {}
    for state, transtions in self.transitions.items():
      enumerated_state = frozenset((states[state],))
      if enumerated_state not in new_transtions: new_transtions[enumerated_state] = {}
      for symbol, next_states in transtions.items():
        if symbol not in new_transtions[enumerated_state]: new_transtions[enumerated_state][symbol] = frozenset()
        for next_state in next_states:
          new_transtions[enumerated_state][symbol] = new_transtions[enumerated_state][symbol].union(frozenset((frozenset((states[next_state],)),)))
    self.transitions = new_transtions
    self.state_token_table = {frozenset((states[state],)): self.state_token_table[state] for state in self.state_token_table}
    return self

  def union(self, other: 'FDA') -> 'FDA':
    '''Enumera e une dois autômatos de forma que o novo autômato mantenha a informação de quais estados originaram de qual autômato.'''
    '''O novo estado inicial é 0, os estados do primeiro autômato são numerados de 1 a n, e os estados do segundo autômato são numerados de n+1 a m+n, onde m é o número de estados do primeiro autômato.'''
    '''O autômato resultante é não determinístico. O algoritmo de determinização utilizará a enumeração dos estados para identificar quais estados representam quais tokens.'''
    a = self.copy().enumerate_states()
    b = other.copy().enumerate_states()
    union = FDA()
    b_start = len(a.states)+1

    union.initial_state = frozenset((0,))
    union.alphabet = a.alphabet.union(b.alphabet)
    # Self states will be shifted by 1 to allow the new initial state as 0,
    # and the other automaton will be shifted by the number of states in the first automaton
    # I'm pretty sure every automata given to this part of the algorithm will have states going from 0 to n-1
    union.states = frozenset(self.add_int_to_state(state, 1) for state in self.states).union(
      frozenset(self.add_int_to_state(state, b_start) for state in other.states)
    ).union(
      frozenset((union.initial_state,))
    )
    union.num_states = len(union.states)
    union.final_states = frozenset(self.add_int_to_state(state, 1) for state in a.final_states).union(
      frozenset(self.add_int_to_state(state, b_start) for state in b.final_states)
    )
    self_transtion = {
      self.add_int_to_state(state, 1): {
        symbol: frozenset(self.add_int_to_state(next_state, 1) for next_state in next_states) for symbol, next_states in a.transitions[state].items()
      } for state in a.transitions 
    }
    other_transition = {
      self.add_int_to_state(state, b_start): {
        symbol: frozenset(self.add_int_to_state(next_state, b_start) for next_state in next_states) for symbol, next_states in b.transitions[state].items()
      } for state in b.transitions
    }
    self_initial_trasitions = {
      union.initial_state: {
        '': frozenset((frozenset((1,)), frozenset((b_start,))))
      }
    }
    union.transitions = {
      **self_transtion,
      **other_transition,
      **self_initial_trasitions
    }
    union.state_token_table = {
      **{self.add_int_to_state(state, 1): a.state_token_table[state] for state in a.state_token_table},
      **{self.add_int_to_state(state, b_start): b.state_token_table[state] for state in b.state_token_table}
    }
    return union

  @staticmethod
  def concat_to_state_name(state: State, string: str) -> State:
    '''Concatena o nome do estado com uma string.'''
    '''Exemplo: um estado "A" vira "0A"'''
    return frozenset(f"{string}{state_part}" for state_part in state)

  @staticmethod
  def add_int_to_state(state: State, num: int) -> State:
    '''Adiciona um número inteiro ao número do estado.'''
    '''O estado deve ser numérico.'''
    '''Exemplo: um estado "0" vira "0+1"'''
    return frozenset(int(state_part)+num for state_part in state)

  @staticmethod
  def state_to_string(state: State, size: int=1) -> str:
    string = ""
    for state_part in state:
      if isinstance(state_part, frozenset):
        for state_part_part in state_part:
          string += str(state_part_part)
      else:
        string += str(state_part)
    number = int(string)
    output = ""
    while len(output) < size:
      output = chr(number % 256) + output
      number //= 256
    return output

  def as_bytes(self) -> bytes:
    '''Serializa o autômato em um vetor de bytes, para exportá-lo em um arquivo binário.'''
    '''Esse arquivo será carregado e parseado em src/fda.rs'''
    output = bytearray()
    state_size = int(math.ceil(math.log(len(self.states))/math.log(256)))
    output.append(state_size)
    for state in self.transitions:
      for symbol in sorted(self.transitions[state]):
        for next_state in sorted(self.transitions[state][symbol]):
          output.append(list(state)[0])
          output.append(ord(symbol))
          output.append(list(next_state)[0])
    return output

  def __str__(self) -> str:
    '''Legado da matéria de formais, não é usado no código atual.'''
    '''Ouput: finals;transitions'''
    state_size = int(math.ceil(math.log(len(self.states))/math.log(256)))
    output = chr(state_size) # Pelo amor de deus se tiver mais que 256^256 estados esse autômato não é pra existir
    output += "".join([self.state_to_string(state, state_size) for state in sorted(self.final_states)]) + chr(255)*state_size
    for state in sorted(self.transitions):
      for symbol in sorted(self.transitions[state]):
        for next_state in sorted(self.transitions[state][symbol]):
          output += f"{self.state_to_string(state, state_size)}{symbol}{self.state_to_string(next_state, state_size)}"
    return output

  def deterministic_equivalent(self) -> 'FDA':
    '''Determiniza o autômato, aqui é considerado que cada estado do novo autômato representará no máximo um único token'''
    '''Caso um estado surja do produto de dois estados do autômato original, o estado resultante representará o token de maior prioridade.'''
    '''A prioridade dos tokens é definida pela ordem em que eles aparecem no arquivo tokens.json e se estão definidos como autômatos ou como strings, os autômatos vêm primeiro.'''
    '''Os tokens estão ordenados de forma que tokens mais genéricos venham antes. Dessa forma, identificadores que poderiam ser intepretados como prefixos de palavras reservadas serão corretamente identificados como identificadores.'''
    '''Exemplo: a string "ret" levaria ao estado não determinístico (id_1, return_2), por ser um id válido mas também estar na terceira letra da palavra reservada "return". Como o id vem primeiro, ele será considerado o representante desse estado.'''
    # Algoritmo de determinização de autômatos não determinísticos
    def epsilon_closure(state: State, closure: set=None) -> State:
      '''Retorna o ε* de um estado, realizando uma busca em profundidade.'''
      if closure is None: closure = set(state)
      if state not in self.transitions or "" not in self.transitions[state]: return frozenset(closure)

      for reachable_state in self.transitions[state][""]:
        if reachable_state not in closure:
          reachable_state_closure: State = epsilon_closure(reachable_state)
          closure.update(reachable_state_closure)

      return frozenset(closure)

    # Se o autômato já é determinístico, retorna uma cópia dele mesmo
    if self.is_deterministic(): return self.copy()

    deterministic = FDA()
    # Trata todo autômato não determinístico como se tivesse transições por ε
    # Caso não tenha, ε* de cada estado é ele mesmo, não influenciando no resultado
    states_epsilon_closure: dict[State, State] = {}
    for state in self.states.difference({'qm'}): states_epsilon_closure[state] = epsilon_closure(state)

    # Construir o autômato determinístico equivalente

    # Conjunto temporário para armaenar os estados a serem incluídos no autômato determinístico
    temp_states: set[State] = set()

    # O estado inicial do autômato determinístico é o ε* do estado inicial do autômato não determinístico
    deterministic.initial_state = states_epsilon_closure[self.initial_state]
    
    # O alfabeto do autômato determinístico é o mesmo do autômato não determinístico, sem o símbolo ε
    deterministic.alphabet = self.alphabet.copy().difference({""})
    
    # Começa a construir as transições do autômato determinístico, partindo do estado inicial
    deterministic.transitions = {}
    stack = [deterministic.initial_state]
    while stack:
      # Adiciona o estado visitado ao conjunto de estados do autômato determinístico
      current_state = stack.pop()
      temp_states.add(current_state)

      # Prepara a tabela de transições para receber o novo estado
      if current_state not in deterministic.transitions:
        deterministic.num_states += 1
        deterministic.transitions[current_state] = {}

      # Para cada símbolo do alfabeto, visita os estados alcançáveis a partir do estado atual
      for symbol in sorted(deterministic.alphabet.difference({""})):
        next_state: set[str] = set()
        for state in sorted(current_state):
          state = frozenset((state,))
          if state not in self.transitions or symbol not in self.transitions[state]: continue
          next_state.update({states_epsilon_closure[huh] for huh in self.transitions[state][symbol]})
        if not next_state: continue
        next_state = frozenset([state_part for x in next_state for state_part in x])
        deterministic.transitions[current_state][symbol] = frozenset([next_state])
        if next_state not in temp_states:
          stack.append(next_state)

    # Agora que todos os estados foram calculados, o conjunto temporário pode ser transformado em um conjunto imutável
    deterministic.states = frozenset(temp_states)
    # Adiciona ao conjunto de estados finais do autômato determinístico os estados que contém algum estado final do autômato não determinístico
    for state in deterministic.states:
      for final_state in self.final_states:
        if final_state.intersection(state):
          deterministic.final_states = deterministic.final_states.union(frozenset((state,)))
          break

    # Adiciona a tabela de estados do autômato determinístico
    deterministic.state_token_table = {}
    for state in deterministic.final_states:
      for state_part in sorted(state):
        # Adiciona na tabela o primeiro estado que for encontrado, de forma que o mais prioritário será o único a ser adicionado
        if frozenset((state_part, )) in self.state_token_table:
          deterministic.state_token_table[state] = self.state_token_table[frozenset((state_part,))]
          break

    return deterministic

  def copy(self) -> 'FDA':
    copy = FDA()
    copy.initial_state = self.initial_state
    copy.final_states = self.final_states.copy()
    copy.states = self.states.copy()
    copy.num_states = self.num_states
    copy.alphabet = self.alphabet.copy()
    copy.transitions = {state: {symbol: next_state.copy() for symbol, next_state in self.transitions[state].items()} for state in self.transitions}
    copy.state_token_table = {state: self.state_token_table[state] for state in self.state_token_table}
    return copy

  def transitions_as_tuples(self) -> list:
    '''Retorna as transições do autômato como uma lista de tuplas (estado, símbolo, próximo estado) para facilitar a ordenação da saída do programa.'''
    transitions = []
    for state in self.transitions:
      for symbol in self.transitions[state]:
        for next_state in self.transitions[state][symbol]:
          transitions.append((state, symbol, next_state))

    transitions.sort(key=lambda x: sorted(x[1])) # Ordena as transições pelo símbolo
    transitions.sort(key=lambda x: sorted(x[0])) # Ordena as transições pelo estado de origem
    return transitions

  def save(self, filename: str) -> None:
    '''Exporta as informações relevantes do autômato.'''
    '''Essas informações estão separadas em duas partes: a lista de transições do autômato e a tabela de estados finais.'''
    '''A lista de transições é serializada para um arquivo binário, no formato estado atual,símbolo,próximo estado.'''
    '''A tabela de estados finais é armazenada num arquivo de texto, no formato estado:token.'''
    state_token_list = ""
    for state in self.state_token_table: state_token_list += f"{list(state)[0]}:{self.state_token_table[state]}\n"
    with open(f"{filename}.automata", "wb") as f: f.write(self.as_bytes())
    with open(f"{filename}_table.automata", "w") as f: f.write(state_token_list)

if __name__ == "__main__":
  def show_automaton(automaton: FDA) -> None:
    '''Mostra o autômato em formato de tabela.'''
    print(f"Estados: {automaton.states}")
    print(f"Estado inicial: {automaton.initial_state}")
    print(f"Estados finais: {automaton.final_states}")
    print(f"Alfabeto: {automaton.alphabet}")
    print("Transições:")
    for state in automaton.transitions:
      for symbol in automaton.transitions[state]:
        for next_state in automaton.transitions[state][symbol]:
          print(f"{state} --{symbol}--> {next_state}")
    print()
