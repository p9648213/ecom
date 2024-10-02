use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub category: String,
    pub brand: String,
    pub price: i64,
    pub sale_price: i64,
    pub total_stock: i64,
}
