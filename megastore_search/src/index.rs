use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};
use crate::product::Product;

pub struct ProductIndex {
    products: HashMap<u32, Arc<Product>>,
    name_index: RwLock<HashMap<String, HashSet<u32>>>,
    brand_index: RwLock<HashMap<String, HashSet<u32>>>,
    category_index: RwLock<HashMap<String, HashSet<u32>>>,
}

impl ProductIndex {
    pub fn new() -> Self {
        ProductIndex {
            products: HashMap::new(),
            name_index: RwLock::new(HashMap::new()),
            brand_index: RwLock::new(HashMap::new()),
            category_index: RwLock::new(HashMap::new()),
        }
    }

    pub fn add_product(&mut self, product: Product) {
        let product_arc = Arc::new(product);
        let id = product_arc.id;
        
        self.products.insert(id, product_arc.clone());
        
        self.add_to_index(&self.name_index, &product_arc.name, id);
        self.add_to_index(&self.brand_index, &product_arc.brand, id);
        self.add_to_index(&self.category_index, &product_arc.category, id);
    }

    fn add_to_index(&self, index: &RwLock<HashMap<String, HashSet<u32>>>, key: &str, id: u32) {
        let key = key.to_lowercase();
        let mut index = index.write().unwrap();
        index.entry(key).or_default().insert(id);
    }
}