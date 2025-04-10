use std::collections::HashMap;
use crate::product::Product;

pub struct SearchEngine {
    index: HashMap<String, Vec<u32>>,
    products: HashMap<u32, Product>,
}

impl SearchEngine {
    pub fn new() -> Self {
        Self {
            index: HashMap::new(),
            products: HashMap::new(),
        }
    }

    pub fn add_product(&mut self, product: Product) {
        let id = product.id;
        for word in Self::tokenize(&product.name) {
            self.index.entry(word).or_default().push(id);
        }
        for word in Self::tokenize(&product.brand) {
            self.index.entry(word).or_default().push(id);
        }
        for word in Self::tokenize(&product.category) {
            self.index.entry(word).or_default().push(id);
        }
        self.products.insert(id, product);
    }

    fn tokenize(text: &str) -> Vec<String> {
        text.to_lowercase()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect()
    }

    pub fn search(&self, name: Option<&str>, brand: Option<&str>, category: Option<&str>) -> Vec<&Product> {
        let mut sets: Vec<Vec<u32>> = Vec::new();

        if let Some(name_filter) = name {
            let words = Self::tokenize(name_filter);
            let ids: Vec<u32> = words
                .iter()
                .flat_map(|w| self.index.get(w).cloned().unwrap_or_default())
                .collect();
            sets.push(ids);
        }

        if let Some(brand_filter) = brand {
            let words = Self::tokenize(brand_filter);
            let ids: Vec<u32> = words
                .iter()
                .flat_map(|w| self.index.get(w).cloned().unwrap_or_default())
                .collect();
            sets.push(ids);
        }

        if let Some(category_filter) = category {
            let words = Self::tokenize(category_filter);
            let ids: Vec<u32> = words
                .iter()
                .flat_map(|w| self.index.get(w).cloned().unwrap_or_default())
                .collect();
            sets.push(ids);
        }

        let final_ids = if sets.is_empty() {
            self.products.keys().cloned().collect()
        } else {
            let mut result = sets[0].clone();
            for s in sets.iter().skip(1) {
                result = result.into_iter().filter(|id| s.contains(id)).collect();
            }
            result
        };

        final_ids
            .iter()
            .filter_map(|id| self.products.get(id))
            .collect()
    }
}
