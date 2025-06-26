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
- [x] Gerar tabela de símbolos para os tokens `id` e `func_id`
  - [x] Armazenar cada entrada
- [x] Perguntar pro Álvaro como serão os erros léxicos (Visto que a maioria dos erros léxicos só são percebidos na análise sintática)
- [x] O autômato criado automaticamente não está guardando referência para quais estados representam quais tokens, é preciso que essa informação seja atualizada sempre que os estados do autômato forem alterados por operações de união/determinização
- [ ] id com ~ tá dando problema; Provavelmente não vamos corrigir isso
## Adicionais
- [x] Substituir if por comandos retornados pelas transições; Os "comandos" foram substituídos pela classificação do token. A ação é sempre determinada pelo tipo do token que acaba de ser lido, logo a existência de comandos específicos para cada transição é desnecessária. Além disso, o único comando observado até o momento foi o de armazenar o valor do token lido junto do tipo. Acredito que no máximo do máximo será feita a distinção para decodificar constantes numéricas em vez de armazenar seus valores como strings.

# Analisador Sintático
## Requisitos
- [x] Gerar árvore sintática
- [x] Gramática da linguagem em LL1
  - [x] Adicionar comentário: A gramática foi modificada para que os identificadores de função possuam uma regex própria, de forma a simplificar a remoção de não determinismo para o valor do ATRIBSTATE; LVALUE = (id | id()) -> LVALUE = (id | func_id())
- [x] Demonstrar que a gramática está em LL1 (Criar arquivo com first e follow)
- [x] Tabela de parsing LL1
## Adicionais
- [x] Usar a posição do token para notificar em que parte do código erros sintáticos acontecem

# Analisador Semântico
## Requisitos
* "A tabela de símbolos" não faz sentido, visto que diferentes escopos podem usar o mesmo símbolo de formas diferentes. Dessa forma, sempre que "a" tabela de símbolos for mencionada, refere-se à tabela adequada na pilha de escopos.
- [x] Pilha de escopos
 - [x] Abrir um novo escopo
 - [x] Fechar o escopo atual
 - [x] Inserir um novo símbolo e seu tipo
  - [x] Verificar se o símbolo já foi inserido no escopo atual (e retornar erro caso positivo)
 - [x] Verificar o típo de um símbolo
  - [x] Buscar o escopo mais ao topo da pilha, retornar erro caso símbolo não tenha sido definido

- [x] Gerar a árvore de expressão com operadores e operandos
- [ ] Inserção de ids na tabela de símbolos
 - [ ] Posição da aparição dos ids está zoada
- [x] Verificação de identificadores por escopo
- [x] Verificação de tipos em expressões numericas. (Talvez em funções)
- [x] Verificar se kw_break está no escopo de um FORSTAT

# Geração de Código Intermediário
- [ ] What the title says

# Entrega
- [ ] Programa com todas as fases
- [x] 3 Programas escritos na linguagem. +100 linhas. (Clara)
- [ ] Makefile: Organizar como o projeto será entregue. Se as dependências já estarão précompiladas ou se os scripts precisarão ser rodados pelo professor.
- [ ] Documentação (Mateus)
 - [x] Documento com prova de que a gramática está em LL1
 - [ ] Documento com prova de que as SDDs são L-atribuídas
- [x] Readme (Sartori)
- [ ] Saída do programa no terminal
 - [ ] Árvores de expressão
  - [ ] Pergutar pro Álvaro se pode ser em um arquivo
 - [ ] Tabela de símbolos com tipo
  - [ ] Perguntar se precisa das posições do símbolo também
 - [ ] Mensagem de sucesso para a análise das expressões
 - [ ] Mensagem de sucesso para a análise das declarações por escopo
 - [ ] Mensagem de sucesso para a análise de comandos por escopo
 - [ ] Código intermediário
  - [ ] Perguntar se pode ser num arquivo de saída
 - [ ] Mensagem detalhada de erro caso haja erro no código fonte. Lembrando que o processo de compilação encerra no momento em que o primeiro erro for encontrado, independentemente da etapa de compilação.
- [x] Cabeçalho com nomes dos integrantes