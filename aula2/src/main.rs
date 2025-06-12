use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]  // Derivando Eq e Hash
struct Produto {
    id: usize,
    nome: String,
}

#[derive(Debug)]
struct Grafo {
    produtos: HashMap<usize, Produto>,
    recomendacoes: HashMap<usize, HashSet<usize>>,
}

impl Grafo {
    fn new() -> Self {
        Grafo {
            produtos: HashMap::new(),
            recomendacoes: HashMap::new(),
        }
    }

    fn adicionar_produto(&mut self, id: usize, nome: String) {
        let produto = Produto { id, nome };
        self.produtos.insert(id, produto);
        self.recomendacoes.insert(id, HashSet::new());
    }

    fn adicionar_recomendacao(&mut self, produto_id1: usize, produto_id2: usize) {
        self.recomendacoes
            .entry(produto_id1)
            .or_insert_with(HashSet::new)
            .insert(produto_id2);
    }

    fn recomendar(&self, produto_id: usize) -> Option<HashSet<Produto>> {
        if let Some(recs) = self.recomendacoes.get(&produto_id) {
            let mut recomendados = HashSet::new();
            for &id in recs {
                if let Some(produto) = self.produtos.get(&id) {
                    recomendados.insert(produto.clone());  // Usando produto.clone() aqui
                }
            }
            Some(recomendados)
        } else {
            None
        }
    }
}

impl Produto {
    fn new(id: usize, nome: &str) -> Self {
        Produto {
            id,
            nome: nome.to_string(),
        }
    }
}

fn main() {
    let mut sistema = Grafo::new();

    // Adicionar produtos ao sistema
    sistema.adicionar_produto(1, "Camiseta".to_string());
    sistema.adicionar_produto(2, "Jaqueta".to_string());
    sistema.adicionar_produto(3, "Tênis".to_string());
    sistema.adicionar_produto(4, "Calça".to_string());

    sistema.adicionar_recomendacao(1, 2); // "Camiseta" -> "Jaqueta"
    sistema.adicionar_recomendacao(1, 3); // "Camiseta" -> "Tênis"
    sistema.adicionar_recomendacao(3, 4); // "Tênis" -> "Calça"

    // Obter recomendações para o produto com id 1 (Camiseta Azul)
    if let Some(recomendacoes) = sistema.recomendar(1) {
        println!("Recomendações para Camiseta:");
        for produto in recomendacoes {
            println!("  - {}", produto.nome);
        }
    } else {
        println!("Nenhuma recomendação encontrada.");
    }

    // Obter recomendações para o produto com id 3 (Tênis Preto)
    if let Some(recomendacoes) = sistema.recomendar(3) {
        println!("Recomendações para Tênis:");
        for produto in recomendacoes {
            println!("  - {}", produto.nome);
        }
    } else {
        println!("Nenhuma recomendação encontrada.");
    }
}


