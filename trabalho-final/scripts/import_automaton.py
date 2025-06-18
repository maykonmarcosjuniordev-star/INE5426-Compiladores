'''Esse é um script de teste para verificar facilmente se a serialização do autômato está correta.'''
'''Supostamente equivalente ao código em src/fda.rs, porém pode estar desatualizado.'''
# THIS IS A TESTING SCRIPT
# This script is only used to test if the automaton serialization is correct
# It may not be up to date with the lastest form of serialization if the tests happened directly in the actual deserialization in the lexer 
# import sys

# if len(sys.argv) != 2:
#   print("Usage: python test.py <string>")
#   sys.exit(1)
# string = sys.argv[1]

transitions = {}

with open("../machines/lexer.automata", "rb") as f:
  state_size = int.from_bytes(f.read(1), byteorder='big')
  while transition := f.read(2*state_size+1):
    current_state, symbol, next_state = transition[:state_size], transition[state_size:state_size+1], transition[state_size+1:]
    cs_n = int.from_bytes(current_state, byteorder='big')
    ns_n = int.from_bytes(next_state, byteorder='big')
    symbol = int.from_bytes(symbol, byteorder='big')
    transitions[(cs_n, chr(symbol))] = ns_n

state_token_list = {}
current_state = 0
with open("../machines/lexer_table.automata", "r") as f:
  while line := f.readline():
    line = line.strip()
    state, token = line.split(":")
    state = int(state)
    if state in state_token_list: print(f"Warning: State {state} already has a token '{state_token_list[state]}', overwriting with '{token}'")
    state_token_list[state] = token

for char in '"Hello world"':
  if (current_state, char) in transitions: current_state = transitions[(current_state, char)]
  elif (current_state, chr(0)) in transitions: current_state = transitions[(current_state, chr(0))]
  else:
    print(f"Error: No transition from state {current_state} with symbol '{char}'")
    current_state = -1
    break
print(current_state, current_state in state_token_list, state_token_list[current_state] if current_state in state_token_list else "None")