# Nota total
min(10, (1.5*AL+2.5*AS+3*ASem+3*GCI+T5))
T5 é ponto extra (0-2) baseado na velocidade do compilador

# Analisador Léxico (1.5 pontos)
## Requisitos
- [-] Gerar lista de tokens
  - [-] Autômato para reconher qualquer token
  - [-] Tabela relacionando estados do autômato aos tokens
  - [-] Tipo do token
  - [-] Valor do token
  - [-] Posição no código (linha, coluna)
- [-] Gerar tabela de símbolos para os tokens `id` e `func_id`
  - [-] Armazenar cada entrada
- [-] Perguntar pro Álvaro como serão os erros léxicos (Visto que a maioria dos erros léxicos só são percebidos na análise sintática)
- [-] O autômato criado automaticamente não está guardando referência para quais estados representam quais tokens, é preciso que essa informação seja atualizada sempre que os estados do autômato forem alterados por operações de união/determinização
## Adicionais
- [-] Substituir if por comandos retornados pelas transições; Os "comandos" foram substituídos pela classificação do token. A ação é sempre determinada pelo tipo do token que acaba de ser lido, logo a existência de comandos específicos para cada transição é desnecessária. Além disso, o único comando observado até o momento foi o de armazenar o valor do token lido junto do tipo. Acredito que no máximo do máximo será feita a distinção para decodificar constantes numéricas em vez de armazenar seus valores como strings.

# Analisador Sintático
## Requisitos
- [-] Gerar árvore sintática
- [-] Gramática da linguagem em LL1
  - [-] Adicionar comentário: A gramática foi modificada para que os identificadores de função possuam uma regex própria, de forma a simplificar a remoção de não determinismo para o valor do ATRIBSTATE; LVALUE = (id | id()) -> LVALUE = (id | func_id())
- [-] Demonstrar que a gramática está em LL1 (Criar arquivo com first e follow)
- [-] Tabela de parsing LL1
## Adicionais
- [-] Usar a posição do token para notificar em que parte do código erros sintáticos acontecem

# Analisador Semântico
## Requisitos
* "A tabela de símbolos" não faz sentido, visto que diferentes escopos podem usar o mesmo símbolo de formas diferentes. Dessa forma, sempre que "a" tabela de símbolos for mencionada, refere-se à tabela adequada na pilha de escopos.
- [-] Pilha de escopos
 - [-] Abrir um novo escopo
 - [-] Fechar o escopo atual
 - [-] Inserir um novo símbolo e seu tipo
  - [-] Verificar se o símbolo já foi inserido no escopo atual (e retornar erro caso positivo)
 - [-] Verificar o típo de um símbolo
  - [-] Buscar o escopo mais ao topo da pilha, retornar erro caso símbolo não tenha sido definido

- [-] Gerar a árvore de expressão com operadores e operandos
- [ ] Inserção de ids na tabela de símbolos
 - [ ] Posição da aparição dos ids está zoada
- [-] Verificação de identificadores por escopo
- [-] Verificação de tipos em expressões numericas. (Talvez em funções)
- [-] Verificar se kw_break está no escopo de um FORSTAT

# Geração de Código Intermediário
- [ ] What the title says

# Entrega
- [ ] Programa com todas as fases
- [-] 3 Programas escritos na linguagem. +100 linhas. (Clara)
- [ ] Makefile: Organizar como o projeto será entregue. Se as dependências já estarão précompiladas ou se os scripts precisarão ser rodados pelo professor.
- [ ] Documentação (Mateus)
 - [-] Documento com prova de que a gramática está em LL1
 - [ ] Documento com prova de que as SDDs são L-atribuídas
- [-] Readme (Sartori)
- [ ] Saída do programa no terminal
 - [ ] Árvores de expressão
  - [ ] Pergutar pro Álvaro se pode ser em um arquivo
 - [-] Tabela de símbolos com tipo
 - [ ] Mensagem de sucesso para a análise das expressões
 - [ ] Mensagem de sucesso para a análise das declarações por escopo
 - [ ] Mensagem de sucesso para a análise de comandos por escopo
 - [ ] Código intermediário
  - [ ] Perguntar se pode ser num arquivo de saída
 - [ ] Mensagem detalhada de erro caso haja erro no código fonte. Lembrando que o processo de compilação encerra no momento em que o primeiro erro for encontrado, independentemente da etapa de compilação.
- [-] Cabeçalho com nomes dos integrantes