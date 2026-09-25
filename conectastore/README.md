# ConectaStore — Sistema de Recomendação de Produtos

## 1. Objetivo

O ConectaStore é um sistema de recomendação de produtos desenvolvido em Rust.

O sistema utiliza um grafo para representar conexões entre produtos. Essas conexões podem representar produtos relacionados ou que possuem alguma similaridade.

A partir de um produto informado pelo usuário, o sistema percorre o grafo e apresenta produtos relacionados como recomendações.

## 2. Funcionamento

O sistema possui um menu interativo com as seguintes funções:

1. Cadastrar produto
2. Listar produtos
3. Criar conexão entre produtos
4. Recomendar produtos
5. Mostrar conexões
6. Medir desempenho
0. Sair

O usuário pode cadastrar produtos informando:

- ID
- Nome
- Categoria

Depois, é possível criar conexões entre dois produtos.

Quando uma recomendação é solicitada, o sistema utiliza o algoritmo BFS (Busca em Largura) para percorrer as conexões do grafo.

## 3. Estruturas de dados utilizadas

### HashMap

O `HashMap` é utilizado para armazenar os produtos.

Cada produto possui um ID utilizado como chave, permitindo localizar rapidamente suas informações.

### Grafo com lista de adjacência

O grafo é representado por um `HashMap<u32, Vec<u32>>`.

Cada ID de produto possui uma lista contendo os produtos conectados a ele.

Essa representação evita a necessidade de criar uma matriz que ocuparia espaço para todas as possíveis combinações entre produtos.

### VecDeque

O `VecDeque` é utilizado como fila durante o algoritmo BFS.

### HashSet

O `HashSet` é utilizado para armazenar os produtos que já foram visitados durante a busca.

Isso evita que o mesmo produto seja processado várias vezes e evita recomendações duplicadas.

## 4. Algoritmo de recomendação

O sistema utiliza BFS (Breadth-First Search), ou Busca em Largura.

O processo funciona da seguinte maneira:

1. O produto informado pelo usuário é colocado na fila.
2. O produto inicial é marcado como visitado.
3. O sistema verifica seus produtos vizinhos.
4. Os produtos ainda não visitados são adicionados à fila.
5. Cada produto encontrado é incluído nas recomendações.
6. O processo continua até que não existam mais produtos para visitar.

## 5. Prevenção de duplicidade

O sistema impede a criação de conexões duplicadas.

Além disso, o `HashSet` utilizado no BFS impede que um mesmo produto seja visitado mais de uma vez durante a recomendação.

## 6. Testes

O projeto possui testes automatizados para verificar:

- Criação de conexões entre produtos.
- Funcionamento das recomendações.

Os testes podem ser executados com:

```bash
cargo test