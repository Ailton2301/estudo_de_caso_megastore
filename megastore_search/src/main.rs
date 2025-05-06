
mod product;
use crate::product::Product;
use std::collections::HashMap;

fn main() {
    let products = vec![
        Product {
            id: 1,
            name: "Smartphone Galaxy A14".to_string(),
            brand: "Samsung".to_string(),
            category: "Eletrônicos".to_string(),
            attributes: HashMap::from([("cor".to_string(), "preto".to_string())]),
            popularity: 0.8,
        },
        Product {
            id: 2,
            name: "Smart TV 55 Polegadas".to_string(),
            brand: "LG".to_string(),
            category: "Eletrônicos".to_string(),
            attributes: HashMap::from([("polegadas".to_string(), "55".to_string())]),
            popularity: 0.9,
        },
    ];

    println!("Busca por 'smartphone':");
    products.iter()
        .filter(|p| p.name.to_lowercase().contains("smartphone"))
        .for_each(|p| println!("{}\nAtributos: {:?}", p, p.attributes));

    println!("\nBusca por 'Samsung':");
    products.iter()
        .filter(|p| p.brand.to_lowercase().contains("samsung"))
        .for_each(|p| println!("{}", p));
}