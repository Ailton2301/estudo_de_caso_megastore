mod product;
mod search_engine;

use product::Product;
use search_engine::SearchEngine;

fn main() {
    let mut engine = SearchEngine::new();

    engine.add_product(Product::new(1, "Smartphone Galaxy A14", "Samsung", "Eletrônicos"));
    engine.add_product(Product::new(2, "Smart TV 55 Polegadas", "LG", "Eletrônicos"));
    engine.add_product(Product::new(3, "Camiseta Esportiva", "Nike", "Vestuário"));
    engine.add_product(Product::new(4, "Tênis de Corrida", "Adidas", "Vestuário"));
    engine.add_product(Product::new(5, "Liquidificador Turbo", "Philco", "Eletrodomésticos"));

    println!("Busca por 'smartphone', marca 'Samsung':");
    let results = engine.search(Some("smartphone"), Some("samsung"), None);
    for product in results {
        println!("{:?}", product);
    }

    println!("\nBusca por categoria 'Vestuário':");
    let results = engine.search(None, None, Some("vestuário"));
    for product in results {
        println!("{:?}", product);
    }

    println!("\nBusca por 'TV':");
    let results = engine.search(Some("TV"), None, None);
    for product in results {
        println!("{:?}", product);
    }
}
