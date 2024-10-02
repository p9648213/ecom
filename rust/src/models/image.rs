use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    pub id: i64,
    pub image: Vec<u8>,
    pub name: String,
    pub content_type: String,
    pub product_id: i64,
}
