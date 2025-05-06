use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub brand: String,
    pub category: String,
    pub attributes: HashMap<String, String>,
    pub popularity: f32,
}

impl fmt::Display for Product {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} ({} - {}) [ID: {}, Popularidade: {:.1}]",
            self.name, self.brand, self.category, self.id, self.popularity
        )
    }
}