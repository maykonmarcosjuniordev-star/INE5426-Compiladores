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
Como diferentes escopos podem usar o mesmo símbolo de formas diferentes, serão criadas várias tabelas de símbolo, uma para cada escopo. Dessa forma, sempre que "a" tabela de símbolos for mencionada, refere-se à tabela adequada na pilha de escopos.
- [-] Pilha de escopos
 - [-] Abrir um novo escopo
 - [-] Fechar o escopo atual
 - [-] Inserir um novo símbolo e seu tipo
  - [-] Verificar se o símbolo já foi inserido no escopo atual (e retornar erro caso positivo)
 - [-] Verificar o típo de um símbolo
  - [-] Buscar o escopo mais ao topo da pilha, retornar erro caso símbolo não tenha sido definido

- [-] Gerar a árvore de expressão com operadores e operandos
  - [ ] Separar as regras sintáticas relacionadas
  - [ ] Adicionar comentários com as regras semânticas
  - [ ] Comparar a implementação com as regras semânticas
- [-] Inserção de ids na tabela de símbolos
  - [ ] Posição da aparição dos ids está zoada
  - [ ] Separar as regras sintáticas relacionadas
  - [ ] Adicionar comentários com as regras semânticas
  - [ ] Comparar a implementação com as regras semânticas
- [-] Verificação de identificadores por escopo
  - [ ] Separar as regras sintáticas relacionadas
  - [ ] Adicionar comentários com as regras semânticas
  - [ ] Comparar a implementação com as regras semânticas
- [-] Verificação de tipos em expressões numericas. (Talvez em funções)
  - [ ] Separar as regras sintáticas relacionadas
  - [ ] Adicionar comentários com as regras semânticas
  - [ ] Comparar a implementação com as regras semânticas
- [-] Verificar se kw_break está no escopo de um FORSTAT

## Adicionais
- [ ] No momento, verificação de tipos é feita utilizando a árvore semântica. Seria mais eficiente realizar essa verificação diretamente na árvore de expressão.

# Geração de Código Intermediário
- [ ] Corrigir cálculo de registradores

# Entrega
- [-] Programa com todas as fases
  - [-] Análise Léxica
  - [-] Análise Sintática
  - [-] Análise Semântica
  - [-] Geração de Código Intermediário (Deus)
- [-] 3 Programas escritos na linguagem. +100 linhas. (Clara)
- [-] Makefile (Maykon)
    - [-] Definir se as dependências já estarão précompiladas ou se os scripts precisarão ser rodados pelo professor.
    - [-] Criar função para execução direta do binário com um arquivo de entrada, de forma não interativa.
    - [-] Retornar toda a saída necessária do programa no terminal. (cat dos arquivos da pasta output)
- [ ] Documentação
 - [-] Documento com prova de que a gramática está em LL1
 - [ ] Documento com prova de que as SDDs são L-atribuídas (Clara)
- [-] Readme
- [ ] Saída do programa no terminal
 - [-] Árvores de expressão
   - [ ] Existem 3 pontos no código no qual o erro semântico não tem acesso à linha e coluna do código fonte na qual o erro acontece. Esses pontos podem ser identificados pela presença de "0, 0" no lugar dos valores apropriados.
 - [-] Tabela de símbolos com tipo
 - [-] Mensagem de sucesso para a análise das expressões
 - [-] Mensagem de sucesso para a análise das declarações por escopo
 - [-] Mensagem de sucesso para a análise de comandos por escopo
 - [-] Código intermediário
 - [-] Mensagem detalhada de erro caso haja erro no código fonte. Lembrando que o processo de compilação encerra no momento em que o primeiro erro for encontrado, independentemente da etapa de compilação.
- [-] Cabeçalho com nomes dos integrantes