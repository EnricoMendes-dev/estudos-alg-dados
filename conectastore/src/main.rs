use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{self, Write};

#[derive(Clone)]
struct Produto {
    id: u32,
    nome: String,
    categoria: String,
}

struct Grafo {
    conexoes: HashMap<u32, Vec<u32>>,
}

impl Grafo {
    fn novo() -> Self {
        Grafo {
            conexoes: HashMap::new(),
        }
    }

    fn adicionar_produto(&mut self, id: u32) {
        self.conexoes.entry(id).or_default();
    }

    fn adicionar_conexao(&mut self, produto1: u32, produto2: u32) {
    if !self
        .conexoes
        .entry(produto1)
        .or_default()
        .contains(&produto2)
    {
        self.conexoes
            .entry(produto1)
            .or_default()
            .push(produto2);
    }

    if !self
        .conexoes
        .entry(produto2)
        .or_default()
        .contains(&produto1)
    {
        self.conexoes
            .entry(produto2)
            .or_default()
            .push(produto1);
    }
}

    fn recomendar(&self, inicio: u32) -> Vec<u32> {
        let mut fila = VecDeque::new();
        let mut visitados = HashSet::new();
        let mut recomendacoes = Vec::new();

        fila.push_back(inicio);
        visitados.insert(inicio);

        while let Some(atual) = fila.pop_front() {
            if let Some(vizinhos) = self.conexoes.get(&atual) {
                for &vizinho in vizinhos {
                    if !visitados.contains(&vizinho) {
                        visitados.insert(vizinho);
                        fila.push_back(vizinho);

                        if vizinho != inicio {
                            recomendacoes.push(vizinho);
                        }
                    }
                }
            }
        }

        recomendacoes
    }
}

fn ler_texto(mensagem: &str) -> String {
    print!("{}", mensagem);
    io::stdout().flush().unwrap();

    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).unwrap();

    entrada.trim().to_string()
}

fn ler_u32(mensagem: &str) -> u32 {
    loop {
        let entrada = ler_texto(mensagem);

        match entrada.parse::<u32>() {
            Ok(numero) => return numero,
            Err(_) => println!("Digite um número válido."),
        }
    }
}

fn cadastrar_produto(
    produtos: &mut HashMap<u32, Produto>,
    grafo: &mut Grafo,
) {
    println!("\n=== CADASTRAR PRODUTO ===");

    let id = ler_u32("ID do produto: ");

    if produtos.contains_key(&id) {
        println!("Esse ID já existe.");
        return;
    }

    let nome = ler_texto("Nome do produto: ");
    let categoria = ler_texto("Categoria: ");

    let produto = Produto {
        id,
        nome,
        categoria,
    };

    produtos.insert(id, produto);
    grafo.adicionar_produto(id);

    println!("Produto cadastrado com sucesso!");
}

fn listar_produtos(produtos: &HashMap<u32, Produto>) {
    println!("\n=== PRODUTOS CADASTRADOS ===");

    if produtos.is_empty() {
        println!("Nenhum produto cadastrado.");
        return;
    }

    let mut lista: Vec<&Produto> = produtos.values().collect();

    lista.sort_by_key(|produto| produto.id);

    for produto in lista {
        println!(
            "ID: {} | {} | Categoria: {}",
            produto.id,
            produto.nome,
            produto.categoria
        );
    }
}

fn criar_conexao(
    produtos: &HashMap<u32, Produto>,
    grafo: &mut Grafo,
) {
    println!("\n=== CRIAR CONEXÃO ===");

    let produto1 = ler_u32("ID do primeiro produto: ");
    let produto2 = ler_u32("ID do segundo produto: ");

    if !produtos.contains_key(&produto1)
        || !produtos.contains_key(&produto2)
    {
        println!("Um ou ambos os produtos não existem.");
        return;
    }

    if produto1 == produto2 {
        println!("Não é possível conectar um produto com ele mesmo.");
        return;
    }

    grafo.adicionar_conexao(produto1, produto2);

    println!("Conexão criada com sucesso!");
}

fn recomendar_produtos(
    produtos: &HashMap<u32, Produto>,
    grafo: &Grafo,
) {
    println!("\n=== RECOMENDAÇÃO DE PRODUTOS ===");

    let id = ler_u32("Digite o ID do produto: ");

    if !produtos.contains_key(&id) {
        println!("Produto não encontrado.");
        return;
    }

    let recomendacoes = grafo.recomendar(id);

    println!("\nProdutos relacionados:");

    if recomendacoes.is_empty() {
        println!("Nenhuma recomendação encontrada.");
        return;
    }

    for recomendacao in recomendacoes {
        if let Some(produto) = produtos.get(&recomendacao) {
            println!(
                "- {} (ID: {}) - {}",
                produto.nome,
                produto.id,
                produto.categoria
            );
        }
    }
}

fn mostrar_conexoes(
    produtos: &HashMap<u32, Produto>,
    grafo: &Grafo,
) {
    println!("\n=== CONEXÕES DO GRAFO ===");

    for (id, conexoes) in &grafo.conexoes {
        if let Some(produto) = produtos.get(id) {
            print!("{} -> ", produto.nome);

            for conexao in conexoes {
                if let Some(produto_conectado) = produtos.get(conexao) {
                    print!("{} | ", produto_conectado.nome);
                }
            }

            println!();
        }
    }
}

use std::time::Instant;

fn medir_desempenho(grafo: &Grafo, quantidade: usize) {
    let inicio = Instant::now();

    for _ in 0..quantidade {
        grafo.recomendar(1);
    }

    let tempo = inicio.elapsed();

    println!(
        "Recomendações: {} | Tempo: {:?}",
        quantidade,
        tempo
    );
}
 fn main() {
    let mut produtos: HashMap<u32, Produto> = HashMap::new();
    let mut grafo = Grafo::novo();

    loop {
        println!("\n================================");
        println!("        CONECTASTORE");
        println!("================================");
        println!("1 - Cadastrar produto");
        println!("2 - Listar produtos");
        println!("3 - Criar conexão entre produtos");
        println!("4 - Recomendar produtos");
        println!("5 - Mostrar conexões");
        println!("6 - Medir desempenho");
        println!("0 - Sair");
        println!("================================");

        let opcao = ler_texto("Escolha uma opção: ");

        match opcao.as_str() {
            "1" => cadastrar_produto(&mut produtos, &mut grafo),
            "2" => listar_produtos(&produtos),
            "3" => criar_conexao(&produtos, &mut grafo),
            "4" => recomendar_produtos(&produtos, &grafo),
            "5" => mostrar_conexoes(&produtos, &grafo),
            "6" => {
                medir_desempenho(&grafo, 100);
                medir_desempenho(&grafo, 1000);
                medir_desempenho(&grafo, 10000);
            }

            "0" => {
                println!("Encerrando o sistema...");
                break;
            }

            _ => println!("Opção inválida."),
        }
    }
}#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testa_conexao_entre_produtos() {
        let mut grafo = Grafo::novo();

        grafo.adicionar_produto(1);
        grafo.adicionar_produto(2);

        grafo.adicionar_conexao(1, 2);

        assert!(grafo.conexoes.get(&1).unwrap().contains(&2));
        assert!(grafo.conexoes.get(&2).unwrap().contains(&1));
    }

    #[test]
    fn testa_recomendacao() {
        let mut grafo = Grafo::novo();

        grafo.adicionar_produto(1);
        grafo.adicionar_produto(2);
        grafo.adicionar_produto(3);

        grafo.adicionar_conexao(1, 2);
        grafo.adicionar_conexao(1, 3);

        let recomendacoes = grafo.recomendar(1);

        assert!(recomendacoes.contains(&2));
        assert!(recomendacoes.contains(&3));
    }
}