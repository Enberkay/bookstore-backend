use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct InventoryResponse {
    pub branch_id: i32,
    pub book_isbn: String,
    pub quantity: i32,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateInventoryRequest {
    pub branch_id: i32,
    pub book_isbn: String,
    pub quantity: i32,
}
