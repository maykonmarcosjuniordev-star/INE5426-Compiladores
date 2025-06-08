# Nota total
min(10, (1.5*AL+2.5*AS+3*ASem+3*GCI+T5))
T5 é ponto extra (0-2) baseado na velocidade do compilador

# Analisador Léxico (1.5 pontos)
## Requisitos
- [x] Gerar lista de tokens
  - [x] Autômato para reconher qualquer token
  - [x] Tabela relacionando estados do autômato aos tokens
  - [x] Tipo do token
  - [x] Valor do token
  - [x] Posição no código (linha, coluna)
- [ ] Gerar tabela de símbolos para os tokens `id`
  - [ ] Armazenar cada entrada
  - [ ] Armazenar dados da entrada
    - [ ] Valor
    - [ ] Tipo
    - [ ] Lista de aparições no código (Vec<u32, u32>)
- [ ] Perguntar pro Álvaro como serão os erros léxicos (Visto que a maioria dos erros léxicos só são percebidos na análise sintática)
- [x] O autômato criado automaticamente não está guardando referência para quais estados representam quais tokens, é preciso que essa informação seja atualizada sempre que os estados do autômato forem alterados por operações de união/determinização
## Adicionais
- [ ] Otimizar a função de transição do autômato finito com uma hash específica.
- [x] Substituir if por comandos retornados pelas transições; Os "comandos" foram substituídos pela classificação do token. A ação é sempre determinada pelo tipo do token que acaba de ser lido, logo a existência de comandos específicos para cada transição é desnecessária. Além disso, o único comando observado até o momento foi o de armazenar o valor do token lido junto do tipo. Acredito que no máximo do máximo será feita a distinção para decodificar constantes numéricas em vez de armazenar seus valores como strings.

# Analisador Sintático
## Requisitos
- [x] Gerar árvore sintática
- [x] Gramática da linguagem em LL1
  - [ ] Adicionar comentário: A gramática foi modificada para que os identificadores de função possuam uma regex própria, de forma a simplificar a remoção de não determinismo para o valor do ATRIBSTATE; LVALUE = (id | id()) -> LVALUE = (id | func_id())
- [ ] Demonstrar que a gramática está em LL1 (Criar arquivo com first e follow)
- [ ] Tabela de parsing LL1
## Adicionais
- [ ] Usar a posição do token para notificar em que parte do código erros sintáticos acontecem

# Analisador Semântico
## Requisitos
- [ ] Gerar a árvore de expressão com operadores e operandos
  - [ ] Perguntar pro Álvaro que porra é essa
- [ ] Inserção de tipos das variáveis (e funções) na tabela de símbolos
- [ ] Verificação de tipos em expressões numericas. (Talvez em funções)
- [ ] Verificação de identificadores por escopo
- [ ] Verificar se kw_break está no escopo de um FORSTAT

# Geração de Código Intermediário
- [ ] What the title says

# Entrega
- [ ] Programa com todas as fases
- [ ] 3 Programas escritos na linguagem. +100 linhas.
- [ ] Makefile
- [ ] Documentação