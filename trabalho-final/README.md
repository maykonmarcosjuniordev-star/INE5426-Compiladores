# INE5426 - Trabalho Final
## Construção de um compilador para a linguagem ConvCC-2025-1
Esse repositório consiste na construção de um compilador para geração de código intermediário da gramática ConvCC-2025-1.

O trabalho foi feito pelos alunos
- Beatriz de Quadros Schmitt - 22100608
- Clara Rosa Oliveira Gonçalves - 22103511
- Gabriel Sartori Rangel - 22100617
- Mateus Goulart Chedid - 22100635
- Maykon Marcos Junior - 22102199

## Modificações na gramática
Para a realização desse projeto, a gramática ConvCC-2025-1 foi levemente modificada, de forma a facilitar o desenvolvimento do analisador sintático. As mudanças são as seguintes:
- Substituição do terminal `id` para identificação da função por um `func_id`. A definição desse token é a mesma que um id normal, porém com um `@` no começo. Essa mudança foi feita para que o não determinismo de `ATRIBSTAT -> EXPRESSION | FUNCCALL` não precisasse ser removido por toda a cadeia da geração de `EXPRESSION`, visto que ambos poderiam começar com `id`.
- Obrigatoriedade de chaves para blocos condicionais para remover a ambiguidade do caso `if cond1 if cond2 statement else statement` que poderia ser interpretado tanto como

    ```c++
    if (cond1) {
    if (cond2) { statement }
    } else { statement }
    ```
    quanto como
    ```c++
    if (cond1) {
    if (cond2) { statement }
    else { statement }
    }
    ```

    Além disso, a nova sintaxe permite encadear else if no mesmo escopo ao invés de colocar o segundo if dentro do escopo do primeiro else.
    ```c++
    // O código
    if (cond1) { statement }
    else {
    if (cond2) { statement}
    }
    // Também pode ser escrito como
    if (cond1) { statement }
    else if (cond2) { statement }
    ```
- Todos os operadores foram agrupados em não terminais relativos ao nível de precedência da operação. <br>Por exemplo: `E -> T + T | T - T` foi definido como `E -> T E_OP T` e `E_OP -> + | -`

## Requisitos
- rust 1.75.0+
- python 3.10+

O projeto foi escrito principalmente na linguagem Rust. Tanto a versão 1.75.0 quanto 1.85.1 (mais recente no momento de entrega do trabalho) compilam sem problemas.

Além disso, o projeto depende de alguns scripts feitos em python3.13. Porém, os scripts se mostraram compatíveis com python3.10+

# Etapas
Os arquivos na pasta `grammars` e na pasta `machines` definem regras para o funcionamento da análise léxica e análise sintática. Alguns desses arquivos foram feitos manualmente e outros são gerados por scripts python da pasta `scripts`. Esses arquivos precisam estar criados no tempo de compilação do projeto, visto que eles serão incorporados no arquivo binário resultante. Assim sendo, após a compilação do projeto, esses arquivos não são mais necessários.

Além disso, vale comentar que não houve preocupação com o desempenho da criação desses arquivos, visto que basta criá-los uma vez previamente ao momento de execução do compilador. Dessa forma, foi possível concentrar esforços em garantir que os arquivos fossem o mais enxutos possíveis e facilmente processados pelo projeto. 

## Análise léxica
A análise léxica utiliza um único autômato capaz de reconhecer e identificar o tokens pertencentes à linguagem. Cada estado do autômato representa um token específico. Uma vez carregado o autômato, lê-se o arquivo de entrada caracter a caracter executando as transições do autômato e armazenando em uma lista de tokens todos os tokens válidos encontrados, que será passada para a análise sintática e impressa na saída do programa.

### Construção do autômato
O arquivo `scripts/generate_lexer.py` é responsável por construir e salvar em `machines/` as informações necessárias para utilização do autômato pelo analisador léxico. Essa parte do trabalho foi feita em python de forma a reutilizar os exercícios realizados anteriormente na matéria de linguagens formais INE5421.

Nesse arquivo, são importados todos os tokens definidos em `grammars/tokens.json` e construídos os autômatos para cada um dos tokens. Esses autômatos são então unidos e determinizados em um único autômato, que é então serializado e armazenado nos arquivos `machines/lexer.automata` e `machines/lexer_table.automata`

Esses dois arquivos serão carregados em `src/fda.rs`. Sendo as transições do autômato armazenadas em `automata.lexer` e a informação de qual estado representa qual token em `lexer_table.automata`

Os algoritmos de união e determinização dos autômatos estão definidos e explicados em `scripts/fda.py`

### Leitura do arquivo de entrada e transições do autômato
A definição do autômato em `src/fda.rs` explica como são executadas as transições do autômato, permitindo que certos estados realizem uma transição por **qualquer** caracter sem necessariamente criar centenas de transições. Além disso, `src/lexer.rs` define como o analisador léxico identifica o final de um token e insere-o na lista de tokens.

Durante a análise léxica, é criada uma tabela de símbolos, que armazena a lista de posições nas quais da token é identificado. Essa tabela também será impressa na saída do programa. Porém, ela não poderá ser utilizada para registro da tipagem dos tokens, visto que um mesmo token pode ser redefinido em diferentes escopos. Dessa forma, o armazenamento da tipagem de tokens será delegado para a análise semântica.

Sobre a distinção entre variáveis e funções: como a definição de funções exige um token específico (sempre começando com `@`), é impossível que uma função e uma variável tenham o mesmo nome.

## Análise sintática
A análise sintática utiliza os arquivos `grammars/syntax.txt` e `grammars/parse-table.txt` para armazenar a lista de regras de geração e a tabela LL1 respectivamente. A prova de que a gramática está em LL(1) está no arquivo `grammars/ll1-proof.txt`

A tabela LL1 foi gerada pelo site [LL(1) parser generator](https://jsmachines.sourceforge.net/machines/ll1.html). Como formato de entrada para esse site, `syntax.txt` é convertido para `syntax-forge.txt` e a tabela resultante foi manualmente escrita em `grammars/parse_table.txt` de forma que `src/syntax.rs` possa facilmente importar e parsear essas informações.

O resultado da análise sintática é uma árvore sintática. Essa estrutura possui uma forma de exportação compatível com o site [GraphVizOnline](https://dreampuf.github.io/GraphvizOnline/?engine=dot), que permite a visualização da árvore na forma de grafo.

## Análise semântica
A implementação das regras será diretamente no código e não haverá necessidade de carregar os arquivos previamente mencionados.

### Pontos relevantes das regras semânticas
#### Tipos de funções
Como a gramática da linguagem é definida de forma que funções não especificam seu tipo de retorno e funções só podem ser chamadas em comandos de atribuição. A regra semântica para ATRIBSTAT -> LVALUE = FUNCCALL. É que LVALUE precisa ser do tipo `int` para ser compatível com a chamada da função.

#### Comando Read
Para receber entrada pelo terminal, é necessário que a variável na qual o valor da entrada será armezado seja uma `string`

### Saída esperada da análise semântica
O resultado da análise semântica consiste de 3 arquivos e 5 mensagens no terminal. As etapas definidas apenas como um arquivo serão armazenadas na pasta output e impressas no terminal. Seguem as definições de cada etapa:
- Construção das Árvores de expressão: Armazenadas em `output/expression_trees/`. Podem ser visualizadas no mesmo site da árvore sintática.
- Inserção de tipo na tabela de símbolos: Implicíta no item "Verificação de Escopos". 
- Verificação de tipos: Mensagem de status no terminal.
- Verificação de identificadores por escopo: Armazenado em `output/scope_stack.log`. No momento em que cada escopo for fechado, será criada uma entrada no log de escopos, listando todos os identificadores definidos no escopo. Isso inclui as informações do identificador pedidas no item "Inserção de tipo na tabela de símbolos".
- Comandos dentro de escopos: Mensagem de status no terminal.

## Geração de Código Intermediário
<!-- TODO: Descrever como isso afeta o programa (depois de implementar) -->
Senta e chora paizão

# Execução do compilador e programas escritos na linguagem
Os arquivos para testar o funcionamento do compilador estão na pasta `inputs`.

Na raiz do projeto, execute caso o projeto ainda não tenha sido compilado:
```
$ make
```

Uma vez compilado, basta usar o comando para rodar o projeto
```
$ make run
```
Ele produz uma CLI, onde é possível selecionar o arquivo a ser testado iterativamente