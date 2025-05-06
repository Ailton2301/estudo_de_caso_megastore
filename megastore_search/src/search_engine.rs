use std::collections::HashMap;
use crate::product::Product;

pub struct SearchEngine {
    products: Vec<Product>,
    cache: HashMap<String, Vec<Product>>,
}

impl SearchEngine {
    pub fn new() -> Self {
        SearchEngine {
            products: Vec::new(),
            cache: HashMap::new(),
        }
    }

    pub fn add_product(&mut self, product: Product) {
        self.products.push(product);
        self.cache.clear(); // limpa o cache quando o catálogo muda
    }

    pub fn search(
        &mut self,
        name: Option<&str>,
        brand: Option<&str>,
        category: Option<&str>,
    ) -> Vec<Product> {
        let key = format!(
            "{}|{}|{}",
            name.unwrap_or("").to_lowercase(),
            brand.unwrap_or("").to_lowercase(),
            category.unwrap_or("").to_lowercase()
        );

        if let Some(cached) = self.cache.get(&key) {
            println!("🔁 Resultado vindo do cache!");
            return cached.clone();
        }

        let results: Vec<Product> = self.products.iter()
            .filter(|product| {
                name.map_or(true, |n| product.name.to_lowercase().contains(&n.to_lowercase())) &&
                brand.map_or(true, |b| product.brand.to_lowercase().contains(&b.to_lowercase())) &&
                category.map_or(true, |c| product.category.to_lowercase().contains(&c.to_lowercase()))
            })
            .cloned()
            .collect();

        self.cache.insert(key, results.clone());

        results
    }
}
